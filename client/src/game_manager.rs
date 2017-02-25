use std::collections::VecDeque;
use sfml::system::Clock;
use common::game::ship::BaseShipBuilder;
use common::game::Game;
use common::protocol::{BufStream, ServerEvent, ServerGame, ClientGame};
use std::io;
use common::serde_json::Error;

pub struct GameManager<S: io::Read + io::Write> {
    clock: Clock,
    ticks: usize,
    skip_ticks: usize,
    stream: BufStream<S>,
    builders: [Vec<BaseShipBuilder>; 2],
    frames: VecDeque<Vec<ServerEvent>>,
}

impl<S: io::Read + io::Write> GameManager<S> {
    pub fn new(builders: [Vec<BaseShipBuilder>; 2], stream: BufStream<S>) -> Self {
        GameManager {
            clock: Clock::new(),
            ticks: 0,
            skip_ticks: 0,
            stream: stream,
            builders: builders,
            frames: VecDeque::new(),
        }
    }
    pub fn do_ticks(&mut self, game: &mut Game) -> Result<(), Error> {
        macro_rules! assert_data {
            ($cond:expr)=>{if !$cond {return Err(io::Error::from(io::ErrorKind::InvalidData).into());}}
        }
        while let Some(msg) = self.stream.read::<ServerGame>() {
            let mut msg = msg?;
            assert_data!(msg.tick>=self.ticks+self.frames.len());
            while msg.tick >= self.ticks + self.frames.len() {
                self.frames.push_back(Vec::new());
            }
            for (tick, evt) in msg.events.drain(..) {
                assert_data!(tick<=msg.tick);
                assert_data!(tick>=self.ticks);
                self.frames[tick - self.ticks].push(evt);
            }
        }
        while self.ticks + self.skip_ticks < self.clock.get_elapsed_time().as_milliseconds() as usize / 20 {
            if let Some(frame) = self.frames.pop_front() {
                for evt in frame.iter() {
                    match *evt {
                        ServerEvent::SpawnShip { player, lane, id } => {
                            assert_data!( player > 1 || lane >= game.lane_count());
                            if let Some(builder_ref) = self.builders[player].get(id) {
                                game.push_ship(builder_ref.build(), player, lane);
                            } else {
                                assert_data!(false);
                            }
                        }
                    }
                }
                game.tick();
                self.ticks += 1;
            } else {
                self.skip_ticks += 1;
            }
        }
        Ok(())
    }
    #[allow(unused_variables)]
    pub fn spawn_ship(&mut self, player: usize, lane: usize, builder_id: usize) -> Result<(), Error> {
        self.stream.write(&ClientGame::SpawnShip { id: builder_id, lane: lane })
    }
}
