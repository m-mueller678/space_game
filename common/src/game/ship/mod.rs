pub mod base_ship;

use super::Position;
use super::Lane;
use graphics;

pub enum Damage {
    Laser(u32),
}

pub trait Ship<T: graphics::RenderTarget> {
    fn position(&self) -> Position;
    fn tick(&mut self, lane: usize, others: &[Lane<T>]);
    fn lane_changed(&mut self, _: &Lane<T>) {}
    fn health(&self) -> u32;
    fn max_health(&self) -> u32;
    fn calc_damage(&self, dmg: &Damage) -> u32;
    fn apply_damage(&mut self, dmg: &Damage);
    #[cfg(feature = "graphics")]
    fn draw(&self, t: &mut T, lane: &Lane<T>);
}
