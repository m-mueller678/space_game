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
    sprite: graphics::Sprite,
}

impl Projectile {
    pub fn new(target: Rc<RefCell<Ship>>, x: i32, y: i32, v: i32, dmg: Damage, sprite: graphics::Sprite) -> Self {
        let weak = Rc::downgrade(&target);
        let target = target.borrow();
        let dx = target.pos_x();
        let dy = target.pos_y();
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
        if let Some(target) = Weak::upgrade(&self.target) {
            let mut target = target.borrow_mut();
            let dx = target.pos_x() - self.pos_x;
            let dy = target.pos_y() - self.pos_y;
            if dx * dx + dy * dy <= 100 {
                target.apply_damage(&self.dmg);
                false
            } else {
                self.pos_x += self.v_x;
                self.pos_y += self.v_y;
                if dx * self.v_x + dy * self.v_y < 0 {
                    self.target = Weak::new();
                }
                true
            }
        } else {
            self.pos_x += self.v_x;
            self.pos_y += self.v_y;
            self.pos_x >= 0 && self.pos_y >= 0 && self.pos_x < game_size_x && self.pos_y < game_size_y
        }
    }
    #[cfg(feature = "graphics")]
    pub fn draw<T: graphics::RenderTarget>(&self, rt: &mut T) {
        use sfml::graphics::RenderStates;
        let mut rs = RenderStates::default();
        rs.transform.translate(self.pos_x as f32, self.pos_y as f32);
        self.sprite.draw(rt, &mut rs);
    }
}