mod ship;
mod lane;

use self::ship::*;
use self::lane::*;

use self::ship::base_ship::BaseShip;


pub struct Game {
    lanes: [Vec<Lane>; 2],
}

impl Game {
    pub fn push_ship<S: Ship + 'static>(&mut self, s: S, direction: usize, lane: usize) {
        self.lanes[direction][lane].push(s);
    }
    pub fn new(size: usize, length: Position) -> Self {
        let mut g = Game {
            lanes: [Vec::with_capacity(size), Vec::with_capacity(size)],
        };
        for i in 0..size {
            g.lanes[0].push(Lane::new(length, i));
            g.lanes[1].push(Lane::new(length, i));
        };
        g
    }
    fn tick(&mut self) {
        let (l1, l2) = self.lanes.split_at_mut(1);
        for l in l1[0].iter_mut() {
            l.tick(&mut l2[0])
        }
        for l in l2[0].iter_mut() {
            l.tick(&mut l1[0])
        }
    }
}