extern crate common;
#[cfg(feature = "graphics")]
extern crate sfml;

use common::game::Game;
use common::game::ship::base_ship::BaseShip;
#[cfg(feature = "graphics")]
use sfml::graphics::*;

#[cfg(not(feature = "graphics"))]
#[test]
fn create_push_tick() {
    let mut g = Game::<()>::new(6, 50);
    g.push_ship(BaseShip::new(), 0, 0);
    for _ in 0..100 {
        g.tick();
    }
}

#[cfg(feature = "graphics")]
#[test]
fn draw() {
    let mut g = Game::new(4, 100_000);
    g.push_ship(BaseShip::new(), 0, 0);
    let mut rt = RenderTexture::new(500, 500, false).unwrap();
    for _ in 0..60 {
        rt.clear(&Color::new_rgb(0, 0, 0));
        g.draw(&mut rt);
        g.tick();
        rt.display()
    }
}