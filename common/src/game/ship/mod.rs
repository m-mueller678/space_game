pub mod base_ship;

use super::Position;
use super::Lane;

pub trait Ship {
    fn position(&self) -> Position;
    fn tick(&mut self, lane: usize, others: &[Lane]);
}
