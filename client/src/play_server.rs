use sfml::graphics::{RenderTarget, RenderWindow, Color};
use sfml::window::event::Event;
use sfml::window::Key;
use common::game::ship::BaseShipBuilder;
use common::protocol::*;
use common::game::Game;
use std::net::TcpStream;
use key_manager::KeyManager;
use game_manager::GameManager;
use game_display::{run as run_display, RunResult};

pub fn run(window: &mut RenderWindow,
           mut stream: BufStream<TcpStream>,
           own_builders: Vec<BaseShipBuilder>,
           keys: &mut KeyManager,
           player: usize)
           -> RunResult {
    let own_start = ClientStart { ships: own_builders };
    if let Err(e) = stream.write(&own_start) {
        return RunResult::IoError(e);
    }
    let other_builders;
    loop {
        match stream.read() {
            Some(Ok(ClientStart { ships })) => {
                other_builders = ships;
                break;
            },
            Some(Err(e)) => {
                return RunResult::IoError(e);
            },
            None => {},
        }
        for evt in window.events() {
            match evt {
                Event::KeyPressed { code: Key::Escape, .. } => return RunResult::Quit,
                Event::Closed => {
                    window.close();
                    return RunResult::Closed;
                },
                _ => {}
            }
        }
        window.clear(&Color::black());
        window.display();
    }
    let mut game = Game::new(4, 10_000);
    let mut game_manager = if player == 0 {
        GameManager::new([own_start.ships, other_builders], stream)
    } else {
        GameManager::new([other_builders, own_start.ships], stream)
    };
    run_display(window, &mut game, &mut game_manager, player, keys)
}
