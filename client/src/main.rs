extern crate common;
extern crate sfml;

mod game_display;
mod game_manager;
mod key_manager;

use common::*;
use common::protocol::BufStream;
use sfml::graphics::*;
use sfml::window::*;
use game_manager::GameManager;
use key_manager::KeyManager;
use std::net::TcpStream;
use std::env::args;

fn main() {
    init_thread_texture_path("./textures/");
    let mut g = game::Game::new(4, 10_000);
    let builder1: game::ship::BaseShipBuilder = serde_json::from_str(include_str!("plasma_ship.json")).unwrap();
    let builder2: game::ship::BaseShipBuilder = serde_json::from_str(include_str!("laser_ship.json")).unwrap();
    let mut window = RenderWindow::new(VideoMode::new_init(600, 600, 32),
                                       "space game",
                                       window_style::CLOSE | window_style::RESIZE,
                                       &ContextSettings::default()).unwrap();
    window.set_framerate_limit(60);
    let mut keys = KeyManager::new();
    let buf_stream = BufStream::new(TcpStream::connect(args().nth(1).unwrap().as_str()).unwrap());
    let mut game_timer = GameManager::new([vec![builder1], vec![builder2]], buf_stream);
    game_display::run(&mut window, &mut g, &mut game_timer, 1, &mut keys);
}