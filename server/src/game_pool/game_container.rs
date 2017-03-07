use std::sync::mpsc::{TryRecvError, Receiver};
use std::mem;
use time::SteadyTime;
use common::game::Game;
use common::game::ship::BaseShipBuilder;
use common::protocol::*;
use server::Stream;
use super::GameStartArg;

pub enum ReadReady {
    Read1,
    Read2,
    None
}

pub struct GameContainer {
    poll: Receiver<ReadReady>,
    streams: [Stream; 2],
    game: Game,
    builders: [Vec<BaseShipBuilder>; 2],
    events: Vec<(usize, ServerEvent)>,
    tick: usize,
    start: SteadyTime,
    last_send: usize,
}

impl GameContainer {
    pub fn new(players: GameStartArg, poll: Receiver<ReadReady>) -> Self {
        GameContainer {
            poll: poll,
            streams: [(players.0).0, (players.1).0],
            game: Game::new(4, 10_000),
            builders: [(players.0).1, (players.1).1],
            events: Vec::new(),
            tick: 0,
            start: SteadyTime::now(),
            last_send: 0,
        }
    }

    pub fn do_work(&mut self) -> bool {
        if !self.update() {
            return false
        }
        loop {
            match self.poll.try_recv() {
                Ok(ReadReady::Read1) => if !self.read(0) { return false },
                Ok(ReadReady::Read2) => if !self.read(1) { return false },
                Ok(ReadReady::None) => {},
                Err(TryRecvError::Empty) => return true,
                Err(TryRecvError::Disconnected) => return false,
            }
        }
    }

    fn read(&mut self, player: usize) -> bool {
        loop {
            match self.streams[player].read() {
                Some(Ok(ClientGame::SpawnShip { id, lane })) => {
                    if lane < self.game.lane_count() && id < self.builders[player].len() {
                        self.game.push_ship(self.builders[player][id].build(), player, lane);
                        self.events.push((self.tick, ServerEvent::SpawnShip { player: player, id: id, lane: lane }));
                        return true;
                    } else {
                        self.streams[player ^ 1].write(&ServerGame::OtherDisconnect).is_ok();
                        return false;
                    }
                },
                Some(Err(_)) => {
                    self.streams[player ^ 1].write(&ServerGame::OtherDisconnect).is_ok();
                    return false;
                },
                None => return true
            }
        }
    }

    fn update(&mut self) -> bool {
        while self.tick < (SteadyTime::now() - self.start).num_milliseconds() as usize / 20 {
            self.game.tick();
            self.tick += 1;
        }
        if self.game.winner().is_some() {
            self.flush_events()
                && self.send_or_disconnect(0, &ServerGame::End)
                && self.send_or_disconnect(1, &ServerGame::End);
            false
        } else if self.tick - self.last_send >= 16 {
            self.flush_events()
        } else {
            true
        }
    }

    fn flush_events(&mut self) -> bool {
        let msg = ServerGame::Update(ServerGameUpdate {
            tick: self.tick,
            events: mem::replace(&mut self.events, Vec::new())
        });
        self.last_send = self.tick;
        self.send_or_disconnect(0, &msg) && self.send_or_disconnect(1, &msg)
    }

    fn send_or_disconnect(&mut self, player: usize, msg: &ServerGame) -> bool {
        if let Err(e) = self.streams[player].write(msg) {
            info! ("error reading from {:?}: {:?}\n\tdisconnect {:?}",
            self.streams[player].raw().peer_addr(), e,
            self.streams[player ^ 1].raw().peer_addr());
            self.streams[player ^ 1].write(&ServerGame::OtherDisconnect).is_ok();
            false
        } else {
            true
        }
    }
}
