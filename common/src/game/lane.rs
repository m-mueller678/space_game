pub type Position = i32;

use std::cell::RefCell;
use std::rc::{Rc, Weak};
use super::ship::*;
use std::ops::*;

pub struct Lane {
    ships: Vec<Rc<RefCell<Ship>>>,
    len: Position,
    pos: usize,
}

impl Lane {
    pub fn flip_pos(&self, pos: Position) -> Position {
        self.len - pos
    }
    pub fn tick(&mut self, other: &mut [Lane]) {
        for s in self.ships.iter_mut() {
            s.borrow_mut().tick(self.pos, other);
        }
    }
}

impl Deref for Lane {
    type Target = [Rc<RefCell<Ship>>];
    fn deref(&self) -> &Self::Target { &self.ships }
}
