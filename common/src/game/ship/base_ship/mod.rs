pub mod builder;

use std::rc::{Weak, Rc};
use std::cell::RefCell;
use game::ship::weapon::*;
use game::ship::{Ship, ShipTrait, Damage};
use game::{Lane, Projectile};
use graphics;

fn mul_frac(m1: u32, m2: u32) -> u32 {
    use std::u32::MAX;
    ((m1 as u64 * m2 as u64) / MAX as u64) as u32
}

pub struct BaseShip {
    target: Weak<RefCell<Ship>>,
    pos: i32,
    pos_y: i32,
    laser_dmg_mult: u32,
    plasma_dmg_mult: u32,
    speed: i32,
    accel: i32,
    max_speed: i32,
    health: u32,
    max_health: u32,
    weapons: Vec<Weapon>,
    #[cfg_attr(not(feature = "graphics"), allow(dead_code))]
    sprite: graphics::CompositeTexture,
}

impl BaseShip {
    fn get_target(&mut self, lane: &Lane) -> Rc<RefCell<Ship>> {
        let mut new_target = lane.mothership();
        let mut min_dist = (lane.mothership().borrow().pos_x() - self.pos).abs();
        for ship in lane.iter() {
            let dist = ship.borrow().pos_x() - self.pos;
            if dist.abs() < min_dist && dist.signum() == self.accel.signum() {
                min_dist = dist.abs();
                new_target = ship;
            }
        }
        self.target = Rc::downgrade(new_target);
        new_target.clone()
    }
    fn do_move(&mut self, m: bool) {
        if m {
            self.speed += self.accel;
            if self.speed.abs() > self.max_speed {
                self.speed = self.max_speed * self.speed.signum();
            }
        } else {
            self.speed /= 2;
        }
        self.pos += self.speed;
    }
    pub fn set_pos_x(&mut self, pos: i32) {
        self.pos = pos;
    }
}

impl ShipTrait for BaseShip {
    fn pos_x(&self) -> i32 {
        self.pos
    }
    fn pos_y(&self) -> i32 {
        self.pos_y
    }
    fn calc_damage(&self, dmg: &Damage) -> u32 {
        match *dmg {
            Damage::Laser(power) => mul_frac(power, self.laser_dmg_mult),
            Damage::Plasma(power) => mul_frac(power, self.plasma_dmg_mult),
        }
    }
    fn apply_damage(&mut self, dmg: &Damage) {
        self.health = self.health.saturating_sub(self.calc_damage(dmg));
    }
    fn health(&self) -> u32 {
        self.health
    }
    fn max_health(&self) -> u32 {
        self.max_health
    }
    fn tick<F: FnMut(Projectile)>(&mut self, lane: usize, others: &[Lane], push_projectile: &mut F) {
        let target_rc = self.get_target(&others[lane]);
        let dist = (target_rc.borrow().pos_x() - self.pos).abs();
        let mut target_args = TickArgs {
            target: target_rc,
            distance: dist,
            push_projectile: push_projectile,
            x: self.pos_x(),
            y: self.pos_y(),
        };
        for w in self.weapons.iter_mut() {
            w.tick(&mut target_args);
        }
        let move_control = self.weapons.iter().map(|x| x.control_move(dist)).sum::<i32>();
        self.do_move(move_control >= 0);
    }

    fn lane_changed(&mut self, l: &Lane) {
        let range = l.y_range();
        self.pos_y = (range.0 + range.1) / 2;
        self.speed = 0;
        self.accel = if l.right_to_left() { -self.accel.abs() } else { self.accel.abs() };
    }
    #[cfg(feature = "graphics")]
    fn draw<T: graphics::RenderTarget>(&self, rt: &mut T, lane: &Lane) {
        use graphics::*;
        let pos_x = self.pos_x() as f32;
        let pos_y = self.pos_y() as f32;
        let cell_rc = Weak::upgrade(&self.target).unwrap_or(lane.mothership().clone());
        let cell_ref = cell_rc.borrow();
        let draw_args = DrawArgs {
            target: &*cell_ref,
            parent: self,
        };
        if self.accel.signum() > 0 {
            let mut render = TransformRender::new(rt, move |(x, y)| (pos_x + x, pos_y + y));
            self.sprite.draw(&mut render);
            for w in self.weapons.iter() {
                w.draw(&mut render, &draw_args);
            }
        } else {
            let mut render = TransformRender::new(rt, move |(x, y)| (pos_x - x, pos_y + y));
            self.sprite.draw(&mut render);
            for w in self.weapons.iter() {
                w.draw(&mut render, &draw_args);
            }
        };

    }
}