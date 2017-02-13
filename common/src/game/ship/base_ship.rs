use super::*;

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
    draw_y_pos: graphics::IfGraphics<i32>,
    laser_dmg_mult: u32,
    health: u32,
    max_health: u32,
}

impl<T: graphics::RenderTarget> BaseShip<T> {
    pub fn new() -> Self {
        return BaseShip {
            target: None,
            pos: 0,
            health: 10000,
            max_health: 10000,
            laser_dmg_mult: 4_000_000_000,
            draw_y_pos: Default::default(),
        };
    }
    fn get_target(&mut self, lane: &Lane<T>) {
        if self.target.as_ref().and_then(|x| Weak::upgrade(&x)).is_none() {
            self.target = lane.iter().filter(|s| {
                let dist = self.pos - s.borrow().position();
                dist >= 0 && dist < 50
            }).next().map(|x| Rc::downgrade(x));
        }
    }
}

impl<T: graphics::RenderTarget> Ship<T> for BaseShip<T> {
    fn position(&self) -> Position {
        self.pos
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
    }
    #[cfg(feature = "graphics")]
    fn lane_changed(&mut self, l: &Lane<T>) {
        use rand::{thread_rng, Rng};
        let range = l.y_range();
        self.draw_y_pos = thread_rng().gen_range(range.0, range.1);
    }
    #[cfg(feature = "graphics")]
    fn draw(&self, target: &mut T, l: &Lane<T>) {
        use sfml::graphics::*;
        let mut circle = CircleShape::new_init(100., 20).unwrap();
        circle.set_origin2f(50., 50.);
        circle.set_position2f(l.draw_pos(self.pos) as f32, self.draw_y_pos as f32);
        circle.draw(target, &mut RenderStates::default());
    }
}