pub mod builder;

use super::*;
use super::weapon::*;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
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
    speed: i32,
    accel: i32,
    max_speed: i32,
    health: u32,
    max_health: u32,
    weapons: Vec<Weapon>,
    draw_move: graphics::IfGraphics<bool>,
    texture: graphics::IfGraphics<Rc<graphics::CompositeTexture>>,
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
        #[cfg(feature = "graphics")]{
            self.draw_move = m;
        }
    }
    pub fn pos_x(&self) -> i32 {
        self.pos
    }
    pub fn set_pos_x(&mut self, pos: i32) {
        self.pos = pos;
    }
    pub fn pos_y(&self) -> i32 {
        self.pos_y
    }
    pub fn calc_damage(&self, dmg: &Damage) -> u32 {
        match *dmg {
            Damage::Laser(power) => mul_frac(power, self.laser_dmg_mult)
        }
    }
    pub fn apply_damage(&mut self, dmg: &Damage) {
        self.health = self.health.saturating_sub(self.calc_damage(dmg));
    }
    pub fn health(&self) -> u32 {
        self.health
    }
    pub fn max_health(&self) -> u32 {
        self.max_health
    }
    pub fn tick(&mut self, lane: usize, others: &[Lane]) {
        let target_rc = self.get_target(&others[lane]);
        let mut cell_ref = target_rc.borrow_mut();
        let dist = (cell_ref.pos_x() - self.pos).abs();
        let mut target_args = TargetArgs {
            ship: &mut *cell_ref,
            distance: dist,
        };
        for w in self.weapons.iter_mut() {
            w.tick(&mut target_args);
        }
        let move_control = self.weapons.iter().map(|x| x.control_move(dist)).sum::<i32>();
        self.do_move(move_control >= 0);
    }

    pub fn lane_changed(&mut self, l: &Lane) {
        #[cfg(feature = "graphics")]{
            use rand::{thread_rng, Rng};
            let range = l.y_range();
            self.pos_y = thread_rng().gen_range(range.0, range.1);
        }
        self.speed = 0;
        self.accel = if l.right_to_left() { self.accel.abs() } else { -self.accel.abs() };
    }
    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T, lane: &Lane) {
        use sfml::graphics::*;

        let mut rs = RenderStates::default();
        rs.transform.translate(self.pos_x() as f32, self.pos_y() as f32);
        self.texture.draw(rt, &mut rs);
        let cell_rc = Weak::upgrade(&self.target).unwrap_or(lane.mothership().clone());
        let cell_ref = cell_rc.borrow();
        let draw_args = DrawArgs {
            target: &*cell_ref,
            parent: self,
        };
        for w in self.weapons.iter() {
            w.draw(rt, &draw_args);
        }
    }
}