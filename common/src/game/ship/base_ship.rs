use super::*;

use std::rc::{Weak, Rc};
use std::cell::RefCell;
use graphics;

pub struct BaseShip<T> {
    target: Option<Weak<RefCell<Ship<T>>>>,
    pos: Position,
    #[cfg(feature = "graphics")]
    draw_y_pos: i32,
}

impl<T: graphics::RenderTarget> BaseShip<T> {
    pub fn new() -> Self {
        #[cfg(feature = "graphics")]
        return BaseShip {
            target: None,
            pos: 0,
            draw_y_pos: 0,
        };
        #[cfg(not(feature = "graphics"))]
        return BaseShip {
            target: None,
            pos: 0,
        };
    }
    fn get_target(&mut self, lane: &Lane<T>) {
        if self.target.as_ref().and_then(|x| Weak::upgrade(&x)).is_none() {
            self.target = lane.iter().filter(|s| {
                let dist = self.pos - s.borrow().position();
                dist >= 0 && dist < 50
            }).next().map(|x| Rc::downgrade(x));
        }
    }
}

impl<T: graphics::RenderTarget> Ship<T> for BaseShip<T> {
    fn position(&self) -> Position {
        self.pos
    }
    fn tick(&mut self, lane: usize, others: &[Lane<T>]) {
        self.get_target(&others[lane]);
        self.pos += 10;
    }
    fn lane_changed(&mut self, l: &Lane<T>) {
        #[cfg(feature = "graphics")]{
            use rand::{thread_rng, Rng};
            let range = l.y_range();
            self.draw_y_pos = thread_rng().gen_range(range.0, range.1);
        }
    }
    #[cfg(feature = "graphics")]
    fn draw(&self, target: &mut T, l: &Lane<T>) {
        use sfml::graphics::*;
        let mut circle = CircleShape::new_init(100., 20).unwrap();
        circle.set_origin2f(50., 50.);
        circle.set_position2f(l.draw_pos(self.pos) as f32, self.draw_y_pos as f32);
        circle.draw(target, &mut RenderStates::default());
    }
}