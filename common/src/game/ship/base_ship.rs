use super::*;
use super::weapon::*;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use graphics;

fn mul_frac(m1: u32, m2: u32) -> u32 {
    use std::u32::MAX;
    ((m1 as u64 * m2 as u64) / MAX as u64) as u32
}

pub struct BaseShip<T> {
    target: Option<Weak<RefCell<Ship<T>>>>,
    pos: Position,
    pos_y: i32,
    laser_dmg_mult: u32,
    health: u32,
    max_health: u32,
    weapons: Vec<Weapon>,
}

impl<T: graphics::RenderTarget> BaseShip<T> {
    pub fn new() -> Self {
        BaseShip {
            target: None,
            pos: 0,
            health: 10000,
            max_health: 10000,
            laser_dmg_mult: 4_000_000_000,
            pos_y: Default::default(),
            weapons: Default::default(),
        }
    }
    fn get_target(&mut self, lane: &Lane<T>) {
        use std::i32;
        if self.target.as_ref().and_then(|x| Weak::upgrade(&x)).is_none() {
            let mut new_target = None;
            let mut min_dist = i32::MAX;
            for ship in lane.iter() {
                let dist = (self.pos - lane.flip_pos(ship.borrow().pos_x())).abs();
                if dist < min_dist {
                    min_dist = dist;
                    new_target = Some(ship);
                }
            }
            self.target = new_target.map(|x| Rc::downgrade(&x));
        }
    }
}

impl<T: graphics::RenderTarget> Ship<T> for BaseShip<T> {
    fn pos_x(&self) -> Position {
        self.pos
    }
    fn pos_y(&self) -> i32 {
        self.pos_y
    }
    fn calc_damage(&self, dmg: &Damage) -> u32 {
        match *dmg {
            Damage::Laser(power) => mul_frac(power, self.laser_dmg_mult)
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
    fn tick(&mut self, lane: usize, others: &[Lane<T>]) {
        self.get_target(&others[lane]);
        self.pos += 10;
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
        } else {
            for w in self.weapons.iter_mut() {
                w.tick::<T>(None);
            }
        };
    }
    #[cfg(feature = "graphics")]
    fn lane_changed(&mut self, l: &Lane<T>) {
        use rand::{thread_rng, Rng};
        let range = l.y_range();
        self.pos_y = thread_rng().gen_range(range.0, range.1);
    }
    #[cfg(feature = "graphics")]
    fn draw(&self, rt: &mut T, l: &Lane<T>) {
        use sfml::graphics::*;
        let mut circle = CircleShape::new_init(100., 20).unwrap();
        circle.set_origin2f(50., 50.);
        circle.set_position2f(l.draw_pos(self.pos) as f32, self.pos_y as f32);
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