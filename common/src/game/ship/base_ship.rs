use super::*;

use std::rc::{Weak, Rc};
use std::cell::RefCell;

pub struct BaseShip {
    target: Option<Weak<RefCell<Ship>>>,
    pos: Position,
}

impl BaseShip {
    pub fn new() -> Self {
        BaseShip {
            target: None,
            pos: 0,
        }
    }
    fn get_target(&mut self, lane: &Lane) {
        if self.target.as_ref().and_then(|x| Weak::upgrade(&x)).is_none() {
            self.target = lane.iter().filter(|s| {
                let dist = self.pos - s.borrow().position();
                dist >= 0 && dist < 50
            }).next().map(|x| Rc::downgrade(x));
        }
    }
}

impl Ship for BaseShip {
    fn position(&self) -> Position {
        self.pos
    }
    fn tick(&mut self, lane: usize, others: &[Lane]) {
        self.get_target(&others[lane]);
        self.pos += 1;
    }
}
