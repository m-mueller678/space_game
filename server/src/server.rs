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
    New { stream: Stream },
    Waiting { stream: Stream, join_id: u32 },
    Preparing { stream: Stream, other_id: usize, second: bool },
    Ready { stream: Stream, other_id: usize, builders: Vec<BaseShipBuilder>, second: bool },
    Playing { game: GameHandle, other_id: usize },

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
                            self.players[p] = PlayerState::New { stream: BufStream::new(stream) }
                        }
                    } else if self.players.len() < MAX_PLAYERS {
                        if let Err(e) = self.poll.register(&stream, Token(self.players.len()), ready, PollOpt::edge()) {
                            error!("cannot register tcp stream {:?} to poll: {:?}", stream, e);
                        } else {
                            info!("put {:?} into slot {}", address, self.players.len());
                            self.players.push(PlayerState::New { stream: BufStream::new(stream) });
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
        let (ret, ps) = match state {
            PlayerState::New { stream } => {
                self.receive_from_new(stream, id)
            },
            PlayerState::Empty => unreachable!(),
            PlayerState::Locked => unreachable!(),
            PlayerState::Waiting { mut stream, join_id } => {
                if stream.read_raw().is_some() {
                    info!("message from {} while waiting", id);
                    (false, PlayerState::Empty)
                } else {
                    (false, PlayerState::Waiting { stream: stream, join_id: join_id })
                }
            },
            PlayerState::Ready { mut stream, other_id, builders, second } => {
                if stream.read_raw().is_some() {
                    info!("message from {} while preparing", id);
                    (false, PlayerState::Empty)
                } else {
                    (false, PlayerState::Ready {
                        stream: stream,
                        other_id: other_id,
                        builders: builders,
                        second: second
                    })
                }
            },

            PlayerState::Preparing { stream, other_id, second } => {
                self.receive_preparing(stream, id, other_id, second)
            },
            PlayerState::Playing { game, other_id } => {
                if game.try_read().is_err() {
                    self.players[other_id] = PlayerState::Empty;
                    (false, PlayerState::Empty)
                } else {
                    (false, PlayerState::Playing { game: game, other_id: other_id })
                }
            }
        };
        self.players[id] = ps;
        ret
    }
    fn receive_preparing(&mut self, mut stream: Stream, id: usize, other_id: usize, second: bool) -> (bool, PlayerState) {
        match stream.read_raw() {
            Some(Ok(raw_msg)) => {
                match from_slice(&raw_msg) {
                    Ok(ClientStart { ships }) => {
                        let other_state = mem::replace(&mut self.players[other_id], PlayerState::Locked);
                        let (ret, ps1, ps2) = match other_state {
                            PlayerState::Preparing { stream: mut stream2, second: second2, other_id } => {
                                if let Err(e) = stream2.write_raw(&raw_msg) {
                                    self.remove_send_err(&stream2, other_id, e);
                                    (false, PlayerState::Empty, PlayerState::Empty)
                                } else {
                                    (true,
                                     PlayerState::Ready {
                                         stream: stream,
                                         builders: ships,
                                         other_id: other_id,
                                         second: second
                                     }, PlayerState::Preparing {
                                        stream: stream2,
                                        other_id: id,
                                        second: second2
                                    })
                                }
                            },
                            PlayerState::Ready { stream: mut stream2, builders: builder2, second: second2, .. } => {
                                if let Err(e) = stream2.write_raw(&raw_msg) {
                                    self.remove_send_err(&stream2, other_id, e);
                                    (false, PlayerState::Empty, PlayerState::Empty)
                                } else {
                                    info!("{} and {} started playing", id, other_id);
                                    if second2 {
                                        let (g1, g2) = self.game_pool.push(((stream, ships), (stream2, builder2)));
                                        (true,
                                         PlayerState::Playing { game: g1, other_id: other_id },
                                         PlayerState::Playing { game: g2, other_id: id })
                                    } else {
                                        let (g2, g1) = self.game_pool.push(((stream2, builder2), (stream, ships)));
                                        (true,
                                         PlayerState::Playing { game: g1, other_id: other_id },
                                         PlayerState::Playing { game: g2, other_id: id })
                                    }
                                }
                            },
                            _ => unreachable!()
                        };
                        self.players[other_id] = ps2;
                        (ret, ps1)
                    },
                    Err(e) => {
                        info!("error parsing builders from {}: {:?}", id, e);
                        self.drop_preparing(other_id);
                        (false, PlayerState::Empty)
                    }
                }
            },
            Some(Err(e)) => {
                info!("io error from {}: {:?}", id, e);
                self.drop_preparing(other_id);
                (false, PlayerState::Empty)
            },
            None => {
                (false, PlayerState::Preparing { stream: stream, other_id: other_id, second: second })
            }
        }
    }
    fn receive_from_new(&mut self, mut stream: Stream, id: usize) -> (bool, PlayerState) {
        match stream.read() {
            Some(Ok(ClientJoin::Join(join_id))) => {
                let pos_opt = self.players.iter().position(|p| match *p {
                    PlayerState::Waiting { join_id: x, .. } if x == join_id => true,
                    _ => false,
                });
                if let Some(id2) = pos_opt {
                    self.join_game(stream, id, id2)
                } else {
                    if self.send_or_remove(id, &mut stream, &ServerJoin::JoinFail) {
                        info!("{} tried to join game {}", id, join_id);
                        (true, PlayerState::New { stream: stream })
                    } else {
                        (false, PlayerState::Empty)
                    }
                }
            },
            Some(Ok(ClientJoin::Create)) => {
                let join_id = id as u32;
                if self.send_or_remove(id, &mut stream, &ServerJoin::Created(join_id)) {
                    info!("{} creates game", id);
                    (true, PlayerState::Waiting { stream: stream, join_id: join_id })
                } else {
                    (false, PlayerState::Empty)
                }
            },
            Some(Err(e)) => {
                info!("error reading from {:?}: {:?}", stream.raw().peer_addr(), e);
                (false, PlayerState::Empty)
            },
            None => (false, PlayerState::New { stream: stream })
        }
    }
    fn join_game(&mut self, mut stream: Stream, id: usize, id2: usize) -> (bool, PlayerState) {
        if let PlayerState::Waiting { stream: mut stream2, .. } = mem::replace(&mut self.players[id2], PlayerState::Locked) {
            if self.send_or_remove(id2, &mut stream2, &ServerJoin::Start(1)) {
                if self.send_or_remove(id, &mut stream, &ServerJoin::Start(0)) {
                    info!("{} joins game created by {}", id, id2);
                    self.players[id2] = PlayerState::Preparing { stream: stream2, other_id: id, second: true };
                    (true, PlayerState::Preparing { stream: stream, other_id: id2, second: false })
                } else {
                    info!("{} was dropped because of failed join by {}", id2, id);
                    self.drop_preparing(id2);
                    (false, PlayerState::Empty)
                }
            } else {
                if self.send_or_remove(id, &mut stream, &ServerJoin::JoinFail) {
                    info!("{} did not respond to join request from {}", id2, id);
                    (true, PlayerState::New { stream: stream })
                } else {
                    (false, PlayerState::Empty)
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
    fn remove(&mut self, id: usize) {
        info!("remove player {}", id);
        match self.players[id] {
            PlayerState::Empty
            | PlayerState::Locked
            | PlayerState::Waiting { .. }
            | PlayerState::New { .. } => {
                self.players[id] = PlayerState::Empty
            },
            PlayerState::Preparing { other_id, .. }
            | PlayerState::Ready { other_id, .. }
            | PlayerState::Playing { other_id, .. } => {
                self.drop_preparing(other_id);
                info!("\tdrop player {}", other_id);
            },
        }
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
                if kind.is_hup() || kind.is_error() {
                    server.remove(id);
                } else {
                    server.drain_stream(id);
                }
            }
        }
    }
}