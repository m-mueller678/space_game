use mio::tcp::TcpStream;
use common::protocol::*;
use common::game::ship::BaseShipBuilder;
use std::sync::mpsc::{Sender, channel};
use threadpool::ThreadPool;

type GameStartArg = ((BufStream<TcpStream>, Vec<BaseShipBuilder>), (BufStream<TcpStream>, Vec<BaseShipBuilder>));

pub struct GameThreadPool {
    pool: ThreadPool
}

impl GameThreadPool {
    pub fn new(thread_count: usize) -> Self {
        GameThreadPool {
            pool: ThreadPool::new(thread_count)
        }
    }
    pub fn push(&mut self, players: GameStartArg) -> (GameContainer, GameContainer) {
        let (send, rec) = channel();
        self.pool.execute(move || inner::run(players, rec));
        (GameContainer {
            sender: send.clone(),
            player_num: 0
        }, GameContainer {
            sender: send,
            player_num: 1
        })
    }
}

#[derive(Debug)]
pub struct GameContainer {
    sender: Sender<usize>,
    player_num: usize,
}

impl GameContainer {
    pub fn try_read(&self) -> Result<(), ()> {
        self.sender.send(self.player_num).map_err(|_| {})
    }
}

mod inner {
    use std::sync::mpsc::{TryRecvError, Receiver};
    use std::mem;
    use time::SteadyTime;
    use common::game::Game;
    use common::game::ship::BaseShipBuilder;
    use common::protocol::*;
    use server::Stream;
    use super::GameStartArg;

    pub fn run(players: GameStartArg, poll: Receiver<usize>) {
        let mut g = InnerGameContainer::new(players, poll);
        while g.update() {
            loop {
                match g.poll.try_recv() {
                    Ok(poll_id) => if !g.read(poll_id) { return },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }
        }
    }

    struct InnerGameContainer {
        poll: Receiver<usize>,
        streams: [Stream; 2],
        game: Game,
        builders: [Vec<BaseShipBuilder>; 2],
        events: Vec<(usize, ServerEvent)>,
        tick: usize,
        start: SteadyTime,
        last_send: usize,
    }

    impl InnerGameContainer {
        fn new(players: GameStartArg, poll: Receiver<usize>) -> Self {
            InnerGameContainer {
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
            if self.tick - self.last_send >= 16 {
                let msg = ServerGame::Update(ServerGameUpdate {
                    tick: self.tick,
                    events: mem::replace(&mut self.events, Vec::new())
                });
                self.last_send = self.tick;
                self.send_or_disconnect(0, &msg) & &self.send_or_disconnect(1, &msg)
            } else {
                true
            }
        }

        fn send_or_disconnect(&mut self, player: usize, msg: &ServerGame) -> bool {
            if self.streams[player].write(msg).is_err() {
                self.streams[player ^ 1].write(&ServerGame::OtherDisconnect).is_ok();
                false
            } else {
                true
            }
        }
    }
}