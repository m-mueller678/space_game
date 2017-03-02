mod game_container;

use std::sync::Arc;
use std::sync::mpsc::{Sender, Receiver, channel, TryRecvError};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use mio::tcp::TcpStream;
use common::protocol::*;
use common::game::ship::BaseShipBuilder;
use self::game_container::GameContainer;

type GameStartArg = ((BufStream<TcpStream>, Vec<BaseShipBuilder>), (BufStream<TcpStream>, Vec<BaseShipBuilder>));

pub struct GameThreadPool {
    threads: Vec<GameThread>,
}

struct GameThread {
    game_count: Arc<AtomicUsize>,
    sender: Sender<(GameStartArg, Receiver<usize>)>,
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
    fn push(&mut self, gsa: GameStartArg, poll_rec: Receiver<usize>) {
        self.sender.send((gsa, poll_rec)).unwrap();
    }
}

fn run_games(rec: Receiver<(GameStartArg, Receiver<usize>)>, game_count: Arc<AtomicUsize>) {
    let mut games = Vec::new();
    loop {
        loop {
            match rec.try_recv() {
                Ok((start_arg, poll_rec)) => {
                    games.push(GameContainer::new(start_arg, poll_rec));
                    game_count.fetch_add(1, Ordering::Relaxed);
                },
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => return,
            }
        }
        let mut i = 0;
        while i < games.len() {
            if games[i].do_work() {
                i += 1;
            } else {
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
    sender: Sender<usize>,
    player_num: usize,
}

impl GameHandle {
    pub fn try_read(&self) -> Result<(), ()> {
        self.sender.send(self.player_num).map_err(|_| {})
    }
}
