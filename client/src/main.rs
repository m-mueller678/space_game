extern crate common;
extern crate sfml;

mod game_display;
mod game_timer;

use common::*;
use sfml::graphics::*;
use sfml::window::*;
use game_timer::GameTimer;

fn main() {
    init_thread_texture_path("./textures/");
    let mut g = game::Game::new(4, 10_000);
    let builder: game::ship::BaseShipBuilder = serde_json::from_str(include_str!("plasma_ship.json")).unwrap();
    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    window.set_framerate_limit(60);
    let mut game_timer = GameTimer::new(builder);
    game_display::run(&mut window, &mut g, &mut game_timer);
}