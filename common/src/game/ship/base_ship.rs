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
    target: Option<Weak<RefCell<Ship>>>,
    pos: i32,
    pos_y: i32,
    laser_dmg_mult: u32,
    speed: i32,
    accel: i32,
    max_speed: i32,
    health: u32,
    max_health: u32,
    weapons: Vec<Weapon>,
    draw_move: graphics::IfGraphics<bool>
}

impl BaseShip {
    pub fn new() -> Self {
        BaseShip {
            target: None,
            pos: 0,
            health: 10000,
            max_health: 10000,
            laser_dmg_mult: 4_000_000_000,
            speed: 0,
            accel: 1,
            max_speed: 20,
            pos_y: 0,
            weapons: vec![Weapon::new_laser(100, 1000, 100)],
            draw_move: Default::default(),
        }
    }
    fn get_target(&mut self, lane: &Lane) {
        use std::i32;
        let mut new_target = None;
        let mut min_dist = i32::MAX;
        for ship in lane.iter() {
            let dist = ship.borrow().pos_x() - self.pos;
            if dist.abs() < min_dist && dist.signum() == self.accel.signum() {
                min_dist = dist.abs();
                new_target = Some(ship);
            }
        }
        self.target = new_target.map(|x| Rc::downgrade(&x));
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
        self.get_target(&others[lane]);
        let cell_rc = self.target.as_ref().and_then(|x| Weak::upgrade(&x));
        if let Some(c) = cell_rc {
            let mut cell_ref = c.borrow_mut();
            let dist = (cell_ref.pos_x() - self.pos).abs();
            let mut target_args = TargetArgs {
                ship: &mut *cell_ref,
                distance: dist,
            };
            for w in self.weapons.iter_mut() {
                w.tick(Some(&mut target_args));
            }
            let move_control = self.weapons.iter().map(|x| x.control_move(dist)).sum::<i32>();
            self.do_move(move_control >= 0);
        } else {
            for w in self.weapons.iter_mut() {
                w.tick(None);
            }
            self.do_move(true);
        };
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
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T, _: &Lane) {
        use sfml::graphics::*;
        let mut circle = CircleShape::new_init(100., 20).unwrap();
        circle.set_origin2f(50., 50.);
        circle.set_position2f(self.pos as f32, self.pos_y as f32);
        circle.draw(rt, &mut RenderStates::default());
        let cell_rc = self.target.as_ref().and_then(|x| Weak::upgrade(&x));
        if let Some(c) = cell_rc {
            let cell_ref = c.borrow();
            let draw_args = DrawArgs {
                target: Some(&*cell_ref),
                parent: self,
            };
            for w in self.weapons.iter() {
                w.draw(rt, &draw_args);
            }
        } else {
            let draw_args = DrawArgs {
                target: None,
                parent: self,
            };
            for w in self.weapons.iter() {
                w.draw(rt, &draw_args);
            }
        };
    }
}