use super::*;

pub struct Mothership {
    x: i32,
    y: i32,
    health: u32
}

impl Mothership {
    pub fn new(x: i32, y: i32) -> Self {
        Mothership {
            x: x,
            y: y,
            health: 10_000,
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

    fn tick(&mut self, _: usize, _: &[Lane]) {}

    fn health(&self) -> u32 {
        self.health
    }

    fn max_health(&self) -> u32 {
        10000
    }

    fn calc_damage(&self, dmg: &Damage) -> u32 {
        match *dmg {
            Damage::Laser(p) => p
        }
    }

    fn apply_damage(&mut self, dmg: &Damage) {
        self.health = self.health.saturating_sub(self.calc_damage(dmg));
    }
    #[cfg(feature = "graphics")]
    fn draw<T: graphics::RenderTarget>(&self, _: &mut T, _: &Lane) {}
}