extern crate common;
extern crate sfml;

use common::*;
use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

fn main() {
    let mut g = game::Game::new(4, 100_000);
    g.push_ship(game::ship::base_ship::BaseShip::new(), 0, 0);
    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    {
        let y = g.size_y() as f32;
        let view = View::new_from_rect(&FloatRect::new(0., 0., y, y)).unwrap();
        window.set_view(&view);
    }
    window.set_framerate_limit(30);
    let mut circle = CircleShape::new_init(0.5, 30).unwrap();
    circle.set_origin(&Vector2f::new(0.5, 0.5));
    while window.is_open() {
        for event in window.events() {
            match event {
                event::Closed => window.close(),
                _ => { /* do nothing */ }
            }
        }
        window.clear(&Color::new_rgb(0, 0, 0));
        g.draw(&mut window);
        g.tick();
        window.display()
    }
}