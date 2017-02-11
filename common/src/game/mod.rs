mod ship;
mod lane;

use self::ship::*;
use self::lane::*;

struct Game {
    lanes: [Vec<Lane>; 2],
}

impl Game {
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