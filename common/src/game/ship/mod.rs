pub mod base_ship;

use super::Position;
use super::Lane;
use graphics;

pub trait Ship<T: graphics::RenderTarget> {
    fn position(&self) -> Position;
    fn tick(&mut self, lane: usize, others: &[Lane<T>]);
    fn lane_changed(&mut self, l: &Lane<T>);
    #[cfg(feature = "graphics")]
    fn draw(&self, t: &mut T, lane: &Lane<T>);
}
