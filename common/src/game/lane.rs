use super::projectile::Projectile;
use std::cell::{RefCell, Cell};
use std::rc::Rc;
use std::ops::FnMut;
use super::ship::*;
use std::ops::*;
#[cfg(feature = "graphics")]
use graphics;

pub const LANE_HEIGHT: i32 = 1000;

pub struct Lane {
    ships: Vec<Rc<RefCell<Ship>>>,
    mothership: Rc<RefCell<Ship>>,
    len: i32,
    pos: usize,
    right_to_left: bool,
}

impl Lane {
    pub fn new(mothership_health: Rc<Cell<u32>>, len: i32, id: usize, right_to_left: bool) -> Self {
        let mothership = Mothership::new(
            mothership_health,
            if right_to_left { len } else { 0 }, id as i32 * LANE_HEIGHT + LANE_HEIGHT / 2
        );
        Lane {
            ships: Vec::new(),
            len: len,
            pos: id,
            right_to_left: right_to_left,
            mothership: Rc::new(RefCell::new(Ship::Mothership(mothership))),
        }
    }
    pub fn push(&mut self, mut s: BaseShip) {
        s.lane_changed(self);
        if self.right_to_left {
            s.set_pos_x(self.len);
        } else {
            s.set_pos_x(0);
        }
        self.ships.push(Rc::new(RefCell::new(Ship::BaseShip(s))));
    }
    pub fn tick<F: FnMut(Projectile)>(&mut self, other: &mut [Lane], push_projectile: &mut F) {
        for s in self.ships.iter_mut() {
            s.borrow_mut().tick(self.pos, other, push_projectile);
        }
        self.ships.retain(|s| s.borrow().health() > 0);
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
    pub fn draw<T: graphics::RenderTarget>(&self, target: &mut T) {
        for s in self.ships.iter() {
            s.borrow().draw(target, self);
        }
    }
    pub fn mothership(&self) -> &Rc<RefCell<Ship>> {
        &self.mothership
    }
}

impl Deref for Lane {
    type Target = [Rc<RefCell<Ship>>];
    fn deref(&self) -> &Self::Target { &self.ships }
}
