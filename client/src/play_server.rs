use sfml::graphics::{RenderWindow};
use sfml::window::event::Event;
use sfml::window::Key;
use common::game::ship::BaseShipBuilder;
use common::protocol::*;
use common::game::Game;
use key_manager::KeyManager;
use game_manager::GameManager;
use game_display::{run as run_display, RunResult};
use std::net::{SocketAddr, TcpStream};
use std::io;


fn check_escape(window: &mut RenderWindow) -> bool {
    for evt in window.events() {
        match evt {
            Event::KeyPressed { code: Key::Escape, .. } => return true,
            Event::Closed => {
                window.close();
                return true;
            },
            _ => {}
        }
    }
    false
}

macro_rules! message_error {
    ($window:expr,$msg:expr)=>{loop{
        match $msg{
            Some(Ok(other))=>{
                return RunResult::IoError(io::Error::new
                (io::ErrorKind::InvalidData,format!("unexpected message: {:?}",other)).into())
            },
            Some(Err(e))=>{
                return RunResult::IoError(e.into())
            },
            None=>{
                if check_escape($window){
                    return RunResult::Quit;
                }
                $window.display();
            }
        }
    }}
}

pub fn server_create(window: &mut RenderWindow,
                     addr: &SocketAddr,
                     own_builders: Vec<BaseShipBuilder>,
                     keys: &mut KeyManager)
                     -> RunResult {
    let mut stream = match create_stream(addr) {
        Ok(stream) => stream,
        Err(e) => return RunResult::IoError(e.into()),
    };
    if let Err(e) = stream.write(&ClientJoin::Create) {
        return RunResult::IoError(e);
    }
    let join_id;
    loop {
        let msg = stream.read();
        if let Some(Ok(ServerJoin::Created(id))) = msg {
            join_id = id;
            info!("created game {} on server {:?}", id, addr);
            break;
        } else {
            message_error!(window,msg)
        }
    }
    println!("{}", join_id);
    let player_num;
    loop {
        let msg = stream.read();
        if let Some(Ok(ServerJoin::Start(player))) = msg {
            player_num = player;
            info!("starting game as {}", player);
            break;
        } else {
            message_error!(window,msg)
        }
    }
    run(window, stream, own_builders, keys, player_num)
}

fn create_stream(addr: &SocketAddr) -> Result<BufStream<TcpStream>, io::Error> {
    let raw_stream = TcpStream::connect(addr)?;
    raw_stream.set_nodelay(true)?;
    Ok(BufStream::new(raw_stream))
}

fn run(window: &mut RenderWindow,
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
        let msg = stream.read();
        if let Some(Ok(ClientStart { ships })) = msg {
            other_builders = ships;
            break;
        } else {
            message_error!(window,msg)
        }
    }
    let mut game = Game::new(4, 10_000);
    let mut game_manager = if player == 0 {
        GameManager::new([own_start.ships, other_builders], stream)
    } else {
        GameManager::new([other_builders, own_start.ships], stream)
    };
    run_display(window, &mut game, &mut game_manager, player, keys)
}
