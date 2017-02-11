pub type Position = i32;

use std::rc::{Rc,Weak};
use super::ship::*;
use std::ops::*;

pub struct Lane {
    ships: Vec<Rc<Ship>>,
    len: Position,
}

impl Lane {
    fn flip_pos(&self, pos: Position) -> Position {
        self.len - pos
    }
}

impl Deref for Lane {
    type Target = [Rc<Ship>];
    fn deref(&self) -> &Self::Target { &self.ships }
}

impl DerefMut for Lane {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.ships }
}
