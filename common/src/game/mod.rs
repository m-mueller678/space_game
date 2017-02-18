pub mod ship;
mod lane;
mod projectile;

use self::projectile::Projectile;
use self::ship::base_ship::BaseShip;
use self::lane::*;
#[cfg(feature = "graphics")]
use graphics;

pub struct Game {
    lanes: [Vec<Lane>; 2],
    projectiles: Vec<Projectile>,
}

impl Game {
    pub fn push_ship(&mut self, s: BaseShip, direction: usize, lane: usize) {
        self.lanes[direction][lane].push(s);
    }
    pub fn new(size: usize, length: i32) -> Self {
        assert!(size > 0);
        let mut g = Game {
            lanes: [Vec::with_capacity(size), Vec::with_capacity(size)],
            projectiles: Vec::new(),
        };
        for i in 0..size {
            g.lanes[0].push(Lane::new(length, i, false));
            g.lanes[1].push(Lane::new(length, i, true));
        };
        g
    }
    pub fn tick(&mut self) {
        {
            let x = self.size_x();
            let y = self.size_y();
            let mut i = 0;
            while i < self.projectiles.len() {
                if self.projectiles[i].tick(x, y) {
                    i += 1;
                } else {
                    self.projectiles.swap_remove(i);
                }
            }
        }
        let (l1, l2) = self.lanes.split_at_mut(1);
        let projectile_ref = &mut self.projectiles;
        for l in l1[0].iter_mut() {
            l.tick(&mut l2[0], &mut |x| projectile_ref.push(x))
        }
        for l in l2[0].iter_mut() {
            l.tick(&mut l1[0], &mut |x| projectile_ref.push(x))
        }
    }
    pub fn lane(&self, direction: usize) -> &[Lane] {
        &self.lanes[direction]
    }
    pub fn size_x(&self) -> i32 {
        self.lanes[0][0].distance()
    }
    pub fn size_y(&self) -> i32 {
        self.lanes[0].len() as i32 * LANE_HEIGHT
    }
    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, target: &mut T) {
        for lvec in self.lanes.iter() {
            for l in lvec.iter() {
                l.draw(target);
            }
        }
        for p in self.projectiles.iter() {
            p.draw(target);
        }
    }
}
