extern crate common;
extern crate sfml;
extern crate env_logger;
#[macro_use]
extern crate log;

mod play_server;
mod game_display;
mod game_manager;
mod key_manager;

use common::*;
use sfml::graphics::*;
use sfml::window::*;
use key_manager::KeyManager;
use std::env::args;

fn main() {
    env_logger::init().unwrap();
    init_thread_texture_path("./textures/");
    let builder1: game::ship::BaseShipBuilder = serde_json::from_str(include_str!("plasma_ship.json")).unwrap();
    let builder2: game::ship::BaseShipBuilder = serde_json::from_str(include_str!("laser_ship.json")).unwrap();
    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    window.set_framerate_limit(60);
    let mut keys = KeyManager::new();
    keys.insert(Key::Z, key_manager::Action::SpawnShip(0));
    println!("{:?}", play_server::server_create(&mut window, &args().nth(1).unwrap().parse().unwrap(), vec![builder1, builder2], &mut keys))
}