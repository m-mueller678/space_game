use std::cell::RefCell;
use std::rc::Rc;
use super::ship::*;
use std::ops::*;
use graphics;

pub const LANE_HEIGHT: i32 = 1000;

pub struct Lane<T> {
    ships: Vec<Rc<RefCell<Ship<T>>>>,
    len: i32,
    pos: usize,
    right_to_left: bool,
}

impl<T: graphics::RenderTarget> Lane<T> {
    pub fn new(len: i32, id: usize, right_to_left: bool) -> Self {
        Lane {
            ships: Vec::new(),
            len: len,
            pos: id,
            right_to_left: right_to_left,
        }
    }
    pub fn push<S: Ship<T> + 'static>(&mut self, mut s: S) {
        s.lane_changed(self);
        if self.right_to_left {
            s.set_pos_x(0);
        } else {
            s.set_pos_x(self.len);
        }
        self.ships.push(Rc::new(RefCell::new(s)));
    }
    pub fn tick(&mut self, other: &mut [Lane<T>]) {
        for s in self.ships.iter_mut() {
            s.borrow_mut().tick(self.pos, other);
        }
    }
    pub fn distance(&self) -> i32 {
        self.len
    }
    pub fn y_range(&self) -> (i32, i32) {
        let base = self.pos as i32 * LANE_HEIGHT;
        (base + LANE_HEIGHT / 5, base + LANE_HEIGHT * 4 / 5)
    }
    pub fn right_to_left(&self) -> bool {
        self.right_to_left
    }
    #[cfg(feature = "graphics")]
    pub fn draw(&self, target: &mut T) {
        for s in self.ships.iter() {
            s.borrow().draw(target, self);
        }
    }
}

impl<T> Deref for Lane<T> {
    type Target = [Rc<RefCell<Ship<T>>>];
    fn deref(&self) -> &Self::Target { &self.ships }
}
