pub mod ship;
mod lane;

use self::ship::*;
use self::lane::*;
use graphics;

pub struct Game<T> {
    lanes: [Vec<Lane<T>>; 2],
}

impl<T: graphics::RenderTarget> Game<T> {
    pub fn push_ship<S: Ship<T> + 'static>(&mut self, s: S, direction: usize, lane: usize) {
        self.lanes[direction][lane].push(s);
    }
    pub fn new(size: usize, length: i32) -> Self {
        assert!(size > 0);
        let mut g = Game {
            lanes: [Vec::with_capacity(size), Vec::with_capacity(size)],
        };
        for i in 0..size {
            g.lanes[0].push(Lane::new(length, i, false));
            g.lanes[1].push(Lane::new(length, i, true));
        };
        g
    }
    pub fn tick(&mut self) {
        let (l1, l2) = self.lanes.split_at_mut(1);
        for l in l1[0].iter_mut() {
            l.tick(&mut l2[0])
        }
        for l in l2[0].iter_mut() {
            l.tick(&mut l1[0])
        }
    }
    pub fn lane(&self, direction: usize) -> &[Lane<T>] {
        &self.lanes[direction]
    }
    pub fn size_x(&self) -> i32 {
        self.lanes[0][0].distance()
    }
    pub fn size_y(&self) -> i32 {
        self.lanes[0].len() as i32 * LANE_HEIGHT
    }
    #[cfg(feature = "graphics")]
    pub fn draw(&self, target: &mut T) {
        for lvec in self.lanes.iter() {
            for l in lvec.iter() {
                l.draw(target);
            }
        }
    }
}
