use super::*;

use std::cell::Cell;
use std::rc::Rc;

pub const MOTHERSHIP_MAX_HEALTH: u32 = 1_000_000;

pub struct Mothership {
    x: i32,
    y: i32,
    health: Rc<Cell<u32>>
}

impl Mothership {
    pub fn new(health: Rc<Cell<u32>>, x: i32, y: i32) -> Self {
        Mothership {
            x: x,
            y: y,
            health: health,
        }
    }
}

impl ShipTrait for Mothership {
    fn pos_x(&self) -> i32 {
        self.x
    }

    fn pos_y(&self) -> i32 {
        self.y
    }

    fn tick<F: FnMut(Projectile)>(&mut self, _: usize, _: &[Lane], _: &mut F) {}

    fn health(&self) -> u32 {
        self.health.get()
    }

    fn max_health(&self) -> u32 {
        MOTHERSHIP_MAX_HEALTH
    }

    fn calc_damage(&self, dmg: &Damage) -> u32 {
        match *dmg {
            Damage::Laser(p) => p,
            Damage::Plasma(p) => p,
        }
    }

    fn apply_damage(&mut self, dmg: &Damage) {
        let new_health = self.health.get().saturating_sub(self.calc_damage(dmg));
        self.health.set(new_health);
    }

    #[cfg(feature = "graphics")]
    fn draw<T: graphics::RenderTarget>(&self, _: &mut T, _: &Lane) {}
}