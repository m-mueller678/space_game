pub type Position = i32;

use std::cell::RefCell;
use std::rc::Rc;
use super::ship::*;
use std::ops::*;

pub struct Lane {
    ships: Vec<Rc<RefCell<Ship>>>,
    len: Position,
    pos: usize,
}

impl Lane {
    pub fn new(len: Position, id: usize) -> Self {
        Lane {
            ships: Vec::new(),
            len: len,
            pos: id,
        }
    }
    pub fn push<S: Ship + 'static>(&mut self, s: S) {
        self.ships.push(Rc::new(RefCell::new(s)));
    }
    pub fn flip_pos(&self, pos: Position) -> Position {
        self.len - pos
    }
    pub fn tick(&mut self, other: &mut [Lane]) {
        for s in self.ships.iter_mut() {
            s.borrow_mut().tick(self.pos, other);
        }
    }
    pub fn distance(&self) -> Position {
        self.len
    }
}

impl Deref for Lane {
    type Target = [Rc<RefCell<Ship>>];
    fn deref(&self) -> &Self::Target { &self.ships }
}
