mod game_container;

use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, channel, RecvTimeoutError};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use time::{SteadyTime, Duration};
use mio::tcp::TcpStream;
use common::protocol::*;
use common::game::ship::BaseShipBuilder;
use self::game_container::{ReadReady, GameContainer};

type GameStartArg = ((BufStream<TcpStream>, Vec<BaseShipBuilder>), (BufStream<TcpStream>, Vec<BaseShipBuilder>));

pub struct GameThreadPool {
    threads: Vec<GameThread>,
}

struct GameThread {
    game_count: Arc<AtomicUsize>,
    sender: Sender<(GameStartArg, Receiver<ReadReady>)>,
}

impl GameThread {
    fn new() -> Self {
        let (send, rec) = channel();
        let r = GameThread {
            game_count: Arc::new(AtomicUsize::new(0)),
            sender: send
        };
        let count_clone = r.game_count.clone();
        thread::spawn(move || run_games(rec, count_clone));
        r
    }
    fn push(&mut self, gsa: GameStartArg, poll_rec: Receiver<ReadReady>) {
        self.sender.send((gsa, poll_rec)).unwrap();
    }
}

fn run_games(rec: Receiver<(GameStartArg, Receiver<ReadReady>)>, game_count: Arc<AtomicUsize>) {
    let mut games = Vec::new();
    loop {
        let rec_end_time = SteadyTime::now() + Duration::milliseconds(10);
        while let Ok(timeout) = (rec_end_time - SteadyTime::now()).to_std() {
            match rec.recv_timeout(timeout) {
                Ok((start_arg, poll_rec)) => {
                    games.push(GameContainer::new(start_arg, poll_rec));
                    game_count.fetch_add(1, Ordering::Relaxed);
                },
                Err(RecvTimeoutError::Timeout) => break,
                Err(RecvTimeoutError::Disconnected) => return,
            }
        }
        let mut i = 0;
        while i < games.len() {
            if games[i].do_work() {
                i += 1;
            } else {
                debug!("removed game container");
                games.swap_remove(i);
                game_count.fetch_sub(1, Ordering::Relaxed);
            }
        }
        thread::yield_now();
    }
}

impl GameThreadPool {
    pub fn new(thread_count: usize) -> Self {
        assert! (thread_count > 0);
        GameThreadPool {
            threads: (0..thread_count).map(|_| { GameThread::new() }).collect()
        }
    }

    pub fn push(&mut self, players: GameStartArg) -> (GameHandle, GameHandle) {
        let (send, rec) = channel();
        let thread = self.threads.iter_mut().min_by_key(|gt| gt.game_count.load(Ordering::Relaxed)).unwrap();
        thread.push(players, rec);
        (GameHandle {
            sender: send.clone(),
            player_num: 0
        }, GameHandle {
            sender: send,
            player_num: 1
        })
    }
}

#[derive(Debug)]
pub struct GameHandle {
    sender: Sender<ReadReady>,
    player_num: usize,
}

impl GameHandle {
    pub fn try_read(&self) -> Result<(), ()> {
        if self.player_num == 0 {
            self.sender.send(ReadReady::Read1).map_err(|_| {})
        } else {
            self.sender.send(ReadReady::Read2).map_err(|_| {})
        }
    }
    pub fn is_active(&self) -> bool {
        self.sender.send(ReadReady::None).is_ok()
    }
}
