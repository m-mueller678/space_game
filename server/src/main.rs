extern crate mio;
extern crate common;
extern crate time;
#[macro_use]
extern crate log;
extern crate env_logger;
extern crate threadpool;

use std::env::args;

mod game_container;
mod server;

pub fn main() {
    env_logger::init().expect("initializing logger");
    server::run(&args().nth(1).expect("expected 1 argument"), 2);
}