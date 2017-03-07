use std::collections::VecDeque;
use sfml::system::Clock;
use common::game::ship::BaseShipBuilder;
use common::game::Game;
use common::protocol::*;
use std::io;
use common::serde_json::Error;
use std::net::TcpStream;

pub struct GameManager {
    end_received: bool,
    clock: Clock,
    skip_ticks: usize,
    stream: BufStream<TcpStream>,
    builders: [Vec<BaseShipBuilder>; 2],
    frames: FrameManager,
}

struct FrameManager {
    frames: VecDeque<Vec<ServerEvent>>,
    next_tick: usize,
}

macro_rules! assert_data {
    ($cond:expr,$reason:expr)=>{
        if !$cond {
            return Err(io::Error::new(io::ErrorKind::InvalidData,$reason).into());
        }
    }
}

impl FrameManager {
    fn end_frame(&self) -> usize {
        self.next_tick + self.frames.len()
    }

    fn push_report(&mut self, mut msg: ServerGameUpdate) -> Result<(), Error> {
        assert_data!(self.end_frame()<=msg.tick,
            format!("server report tick {} before previous report {}",msg.tick,self.end_frame()));
        while self.end_frame() < msg.tick {
            self.frames.push_back(Vec::new());
        }
        for (tick, evt) in msg.events.drain(..) {
            assert_data!(tick<msg.tick,"server event past parent report");
            assert_data!(tick>=self.next_tick,"server event tick before previous report");
            self.frames[tick - self.next_tick].push(evt);
        }
        Ok(())
    }

    fn try_tick(&mut self, game: &mut Game, builders: &[Vec<BaseShipBuilder>; 2]) -> Result<bool, Error> {
        if let Some(frame) = self.frames.pop_front() {
            for evt in frame.iter() {
                match *evt {
                    ServerEvent::SpawnShip { player, lane, id } => {
                        assert_data!( player <= 1,"invalid player in SpawnShip event");
                        assert_data!( lane < game.lane_count(),"invalid lane in SpawnShip event");
                        if let Some(builder_ref) = builders[player].get(id) {
                            game.push_ship(builder_ref.build(), player, lane);
                        } else {
                            assert_data!(false,"invalid ship in SpawnShip event");
                        }
                    }
                }
            }
            game.tick();
            self.next_tick += 1;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

impl GameManager {
    pub fn new(builders: [Vec<BaseShipBuilder>; 2], stream: BufStream<TcpStream>) -> Self {
        GameManager {
            end_received: false,
            clock: Clock::new(),
            skip_ticks: 0,
            stream: stream,
            builders: builders,
            frames: FrameManager {
                next_tick: 0,
                frames: VecDeque::new(),
            }
        }
    }
    pub fn do_ticks(&mut self, game: &mut Game) -> Result<bool, Error> {
        while !self.end_received {
            match self.stream.read() {
                Some(Ok(ServerGame::Update(msg))) => {
                    self.frames.push_report(msg)?;
                },
                Some(Ok(ServerGame::OtherDisconnect)) => {
                    use common::serde::de::Error;
                    return Err(Error::custom("other disconnect"))
                },
                Some(Ok(ServerGame::End)) => {
                    self.end_received = true;
                }
                Some(Err(e)) => return Err(e),
                None => break
            }
        }
        while self.frames.next_tick + self.skip_ticks < self.clock.get_elapsed_time().as_milliseconds() as usize / 20 {
            if !self.frames.try_tick(game, &self.builders)? {
                self.skip_ticks += 1;
            }
        }
        Ok(self.end_received && self.frames.frames.is_empty())
    }
    #[allow(unused_variables)]
    pub fn spawn_ship(&mut self, player: usize, lane: usize, builder_id: usize) -> Result<(), Error> {
        self.stream.write(&ClientGame::SpawnShip { id: builder_id, lane: lane })
    }
}
