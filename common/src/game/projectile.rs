use super::ship::*;
use std::rc::{Weak, Rc};
use std::cell::RefCell;
use std::cmp::max;
use graphics;

pub struct Projectile {
    target: Weak<RefCell<Ship>>,
    pos_x: i32,
    pos_y: i32,
    v_x: i32,
    v_y: i32,
    dmg: Damage,
    #[cfg_attr(not(feature = "graphics"), allow(dead_code))]
    sprite: graphics::Sprite,
}

fn dot_p(v1: &[i32; 2], v2: &[i32; 2]) -> i32 {
    v1[0] * v2[0] + v1[1] * v2[1]
}

fn cross_p_scalar(v1: &[i64; 2], v2: &[i64; 2]) -> i64 {
    v1[0] * v2[1] - v1[1] * v2[0]
}

fn collides(target: &[i64; 2], path: &[i64; 2], radius: i64) -> bool {
    let move_len_sq = path[0] * path[0] + path[1] * path[1];
    //area of parallelogram defined by points: start, end, target
    let move_area = cross_p_scalar(target, path) as i64;
    let min_dist_sq = move_area.saturating_mul(move_area) / move_len_sq;
    min_dist_sq <= radius * radius
}

impl Projectile {
    pub fn new(target: Rc<RefCell<Ship>>, x: i32, y: i32, v: i32, dmg: Damage, sprite: graphics::Sprite) -> Self {
        let weak = Rc::downgrade(&target);
        let target = target.borrow();
        let dx = target.pos_x() - x;
        let dy = target.pos_y() - y;
        let hyp = max((dx as f64).hypot(dy as f64) as i32, 1);
        Projectile {
            target: weak,
            pos_x: x,
            pos_y: y,
            v_x: dx * v / hyp,
            v_y: dy * v / hyp,
            dmg: dmg,
            sprite: sprite,
        }
    }
    pub fn tick(&mut self, game_size_x: i32, game_size_y: i32) -> bool {
        self.pos_x += self.v_x;
        self.pos_y += self.v_y;
        if let Some(target) = Weak::upgrade(&self.target) {
            let mut target = target.borrow_mut();
            let dx = target.pos_x() - self.pos_x;
            let dy = target.pos_y() - self.pos_y;
            if dot_p(&[dx, dy], &[self.v_x, self.v_y]) < 0 {
                if collides(&[dx as i64, dy as i64], &[-self.v_x as i64, -self.v_y as i64], 20) {
                    target.apply_damage(&self.dmg);
                    false
                } else {
                    self.target = Weak::new();
                    true
                }
            } else {
                true
            }
        } else {
            self.pos_x >= 0 && self.pos_y >= 0 && self.pos_x < game_size_x && self.pos_y < game_size_y
        }
    }
    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T) {
        use graphics::TransformRender;
        let (pos_x, pos_y) = (self.pos_x, self.pos_y);
        self.sprite.draw(&mut TransformRender::new(rt, move |(x, y)| (pos_x as f32 + x, pos_y as f32 + y)));
    }
}