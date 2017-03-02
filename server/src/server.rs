use std::net::SocketAddr;
use std::io::ErrorKind;
use std::str::FromStr;
use std::mem;
use mio::tcp::{TcpStream, TcpListener};
use mio::{Poll, Token, Ready, PollOpt, Events};
use common::protocol::*;
use game_pool::{GameHandle, GameThreadPool};
use common::game::ship::BaseShipBuilder;
use common::serde_json::from_slice;
use common::serde::Serialize;
use std::fmt::Debug;

pub type Stream = BufStream<TcpStream>;

#[derive(Debug)]
enum PlayerState {
    Empty,
    New(Stream),
    Waiting(Stream, u32),
    Preparing(Stream, usize, bool),
    Ready(Stream, Vec<BaseShipBuilder>, bool),
    Playing(GameHandle, usize),

    Locked,
}

struct Server {
    players: Vec<PlayerState>,
    listener: TcpListener,
    poll: Poll,
    game_pool: GameThreadPool,
}

impl Server {
    fn drain_listener(&mut self) {
        loop {
            match self.listener.accept() {
                Ok((stream, address)) => {
                    let pos = self.players.iter().position(|p| match *p {
                        PlayerState::Empty => true,
                        _ => false
                    });
                    let ready = Ready::readable() | Ready::error() | Ready::hup();
                    if let Some(p) = pos {
                        if let Err(e) = self.poll.register(&stream, Token(p), ready, PollOpt::edge()) {
                            error!("cannot register tcp stream {:?} to poll: {:?}", stream, e);
                        } else {
                            info!("put {:?} into slot {}", address, p);
                            self.players[p] = PlayerState::New(BufStream::new(stream))
                        }
                    } else if self.players.len() < MAX_PLAYERS {
                        if let Err(e) = self.poll.register(&stream, Token(self.players.len()), ready, PollOpt::edge()) {
                            error!("cannot register tcp stream {:?} to poll: {:?}", stream, e);
                        } else {
                            info!("put {:?} into slot {}", address, self.players.len());
                            self.players.push(PlayerState::New(BufStream::new(stream)));
                        }
                    } else {
                        //drop stream
                    }
                },
                Err(e) => {
                    if e.kind() == ErrorKind::WouldBlock {
                        break;
                    } else {
                        panic!("listener error: {:?}", e)
                    }
                },
            }
        }
    }
    fn drain_stream(&mut self, id: usize) {
        while self.try_read_stream(id) {}
        debug!("players: {:?}", self.players);
    }
    fn try_read_stream(&mut self, id: usize) -> bool {
        let state = mem::replace(&mut self.players[id], PlayerState::Locked);
        match state {
            PlayerState::New(stream) => {
                self.receive_from_new(stream, id)
            },
            PlayerState::Empty => unreachable!(),
            PlayerState::Locked => unreachable!(),
            PlayerState::Waiting(mut stream, join_id) => {
                if stream.read_raw().is_some() {
                    info!("message from {} while waiting", id);
                    self.players[id] = PlayerState::Empty;
                } else {
                    self.players[id] = PlayerState::Waiting(stream, join_id);
                }
                false
            },
            PlayerState::Ready(mut stream, builders, second) => {
                if stream.read_raw().is_some() {
                    info!("message from {} while waiting", id);
                    self.players[id] = PlayerState::Empty;
                } else {
                    self.players[id] = PlayerState::Ready(stream, builders, second);
                }
                false
            },

            PlayerState::Preparing(stream, other_id, second) => {
                self.receive_preparing(stream, id, other_id, second)
            },
            PlayerState::Playing(game, other_id) => {
                if game.try_read().is_err() {
                    self.players[id] = PlayerState::Empty;
                    self.players[other_id] = PlayerState::Empty;
                } else {
                    self.players[id] = PlayerState::Playing(game, other_id);
                }
                false
            }
        }
    }
    fn receive_preparing(&mut self, mut stream: Stream, id: usize, other_id: usize, second: bool) -> bool {
        match stream.read_raw() {
            Some(Ok(raw_msg)) => {
                match from_slice(&raw_msg) {
                    Ok(ClientStart { ships }) => {
                        let other_state = mem::replace(&mut self.players[other_id], PlayerState::Locked);
                        match other_state {
                            PlayerState::Preparing(mut stream2, _, second2) => {
                                if let Err(e) = stream2.write_raw(&raw_msg) {
                                    self.remove_send_err(&stream2, other_id, e);
                                    self.drop_preparing(id);
                                    false
                                } else {
                                    self.players[id] = PlayerState::Ready(stream, ships, !second2);
                                    self.players[other_id] = PlayerState::Preparing(stream2, id, second2);
                                    true
                                }
                            },
                            PlayerState::Ready(mut stream2, builder2, second2) => {
                                if let Err(e) = stream2.write_raw(&raw_msg) {
                                    self.remove_send_err(&stream2, other_id, e);
                                    self.drop_preparing(id);
                                    false
                                } else {
                                    if second2 {
                                        let (g1, g2) = self.game_pool.push(((stream, ships), (stream2, builder2)));
                                        self.players[id] = PlayerState::Playing(g1, other_id);
                                        self.players[other_id] = PlayerState::Playing(g2, id);
                                    } else {
                                        let (g2, g1) = self.game_pool.push(((stream2, builder2), (stream, ships)));
                                        self.players[id] = PlayerState::Playing(g1, other_id);
                                        self.players[other_id] = PlayerState::Playing(g2, id);
                                    };
                                    info!("{} and {} started playing", id, other_id);
                                    true
                                }
                            },
                            _ => unreachable!()
                        }
                    },
                    Err(e) => {
                        info!("error parsing builders from {}: {:?}", id, e);
                        self.players[id] = PlayerState::Empty;
                        self.drop_preparing(other_id);
                        false
                    }
                }
            },
            Some(Err(e)) => {
                info!("io error from {}: {:?}", id, e);
                self.players[id] = PlayerState::Empty;
                self.drop_preparing(other_id);
                false
            },
            None => {
                self.players[id] = PlayerState::Preparing(stream, other_id, second);
                false
            }
        }
    }
    fn receive_from_new(&mut self, mut stream: Stream, id: usize) -> bool {
        match stream.read() {
            Some(Ok(ClientJoin::Join(join_id))) => {
                let pos_opt = self.players.iter().position(|p| match *p {
                    PlayerState::Waiting(_, x) if x == join_id => true,
                    _ => false,
                });
                if let Some(id2) = pos_opt {
                    self.join_game(stream, id, id2)
                } else {
                    if self.send_or_remove(id, &mut stream, &ServerJoin::JoinFail) {
                        info!("{} tried to join game {}", id, join_id);
                        self.players[id] = PlayerState::New(stream);
                        true
                    } else {
                        false
                    }
                }
            },
            Some(Ok(ClientJoin::Create)) => {
                let join_id = id as u32;
                if self.send_or_remove(id, &mut stream, &ServerJoin::Created(join_id)) {
                    info!("{} creates game", id);
                    self.players[id] = PlayerState::Waiting(stream, join_id);
                    true
                } else {
                    false
                }
            },
            Some(Err(e)) => {
                info!("error reading from {:?}: {:?}", stream.raw().peer_addr(), e);
                self.players[id] = PlayerState::Empty;
                false
            },
            None => {
                self.players[id] = PlayerState::New(stream);
                false
            }
        }
    }
    fn join_game(&mut self, mut stream: Stream, id: usize, id2: usize) -> bool {
        if let PlayerState::Waiting(mut stream2, _) = mem::replace(&mut self.players[id2], PlayerState::Locked) {
            if self.send_or_remove(id2, &mut stream2, &ServerJoin::Start(1)) {
                if self.send_or_remove(id, &mut stream, &ServerJoin::Start(0)) {
                    info!("{} joins game created by {}", id, id2);
                    self.players[id2] = PlayerState::Preparing(stream2, id, true);
                    self.players[id] = PlayerState::Preparing(stream, id2, false);
                    true
                } else {
                    info!("{} was dropped because of failed join by {}", id2, id);
                    self.drop_preparing(id2);
                    false
                }
            } else {
                if self.send_or_remove(id, &mut stream, &ServerJoin::JoinFail) {
                    info!("{} did not respond to join request from {}", id2, id);
                    self.players[id] = PlayerState::New(stream);
                    true
                } else {
                    false
                }
            }
        } else {
            unreachable!();
        }
    }
    fn send_or_remove<Msg: Serialize>(&mut self, id: usize, stream: &mut Stream, msg: &Msg) -> bool {
        if let Err(e) = stream.write(msg) {
            self.remove_send_err(stream, id, e);
            false
        } else {
            true
        }
    }
    fn remove_send_err<E: Debug>(&mut self, stream: &Stream, id: usize, e: E) {
        info!("error sending to {:?}: {:?}", stream.raw().peer_addr(), e);
        self.players[id] = PlayerState::Empty;
    }
    fn drop_preparing(&mut self, id: usize) {
        self.players[id] = PlayerState::Empty;
    }
}

const MAX_PLAYERS: usize = 1000;
const LISTENER_TOKEN: usize = 1000;

pub fn run(address: &str, num_threads: usize) -> ! {
    let parsed_address = SocketAddr::from_str(address).expect(&format!("parsing socket address {:?}", address));
    let mut server = Server {
        game_pool: GameThreadPool::new(num_threads),
        players: Vec::new(),
        listener: TcpListener::bind(&parsed_address).expect(&format!("cannot open tcp listener for {:?}", parsed_address)),
        poll: Poll::new().expect("cannot create poll"),
    };
    server.poll.register(&server.listener, Token(LISTENER_TOKEN), Ready::readable() | Ready::hup() | Ready::error(), PollOpt::edge())
        .expect("cannot register tcp listener to poll");
    let mut events = Events::with_capacity(256);
    loop {
        server.poll.poll(&mut events, None).expect("polling");
        for e in events.iter() {
            let kind = e.kind();
            let id = e.token().0;
            if id == LISTENER_TOKEN {
                if kind.is_readable() {
                    server.drain_listener();
                } else {
                    panic!("listener error: {:?}", server.listener.take_error());
                }
            } else {
                server.drain_stream(id);
            }
        }
    }
}