extern crate common;
extern crate sfml;

use common::*;
use common::game::ship::Ship;
use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

fn main() {
    let mut g = game::Game::new(4, 1000);
    g.push_ship(game::ship::base_ship::BaseShip::new(), 0, 0);

    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    window.set_view(&View::new_init(&Vector2f::new(0.5, 0.5), &Vector2f::new(1., 1.)).unwrap());
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
        for d in 0..1 {
            for l in g.lane(d).iter() {
                for ship in l.iter().map(|x| x.borrow()) {
                    let p = if d == 0 { ship.position() } else { l.flip_pos(ship.position()) };
                    circle.set_position(&Vector2f::new(p as f32 / l.distance() as f32, 0.5));
                    window.draw(&circle);
                }
            }
        }
        g.tick();
        window.display()
    }
}