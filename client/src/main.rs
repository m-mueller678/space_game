extern crate common;
extern crate sfml;

use common::*;
use sfml::graphics::*;
use sfml::window::*;
use sfml::system::*;

fn main() {
    init_thread_texture_path("./textures/");
    let builder = include_str!("plasma_ship.json");
    let builder: game::ship::BaseShipBuilder = serde_json::from_str(builder).unwrap();
    let mut g = game::Game::new(4, 10_000);
    g.push_ship(builder.build(), 0, 0);
    g.push_ship(builder.build(), 1, 0);
    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    {
        let view = View::new_from_rect(&FloatRect::new(0., 0., g.size_x() as f32, g.size_y() as f32)).unwrap();
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