use sfml::system::Clock;
use common::game::ship::BaseShipBuilder;
use common::game::Game;

pub struct GameManager {
    clock: Clock,
    ticks: u32,
    builder1: BaseShipBuilder,
    builder2: BaseShipBuilder,
}

impl GameManager {
    pub fn new(builder1: BaseShipBuilder, builder2: BaseShipBuilder) -> Self {
        GameManager {
            clock: Clock::new(),
            ticks: 0,
            builder1: builder1,
            builder2: builder2,
        }
    }
    pub fn do_ticks(&mut self, game: &mut Game) {
        while self.ticks < self.clock.get_elapsed_time().as_milliseconds() as u32 / 20 {
            if self.ticks % 128 == 0 {
                if self.ticks as usize / 128 % 2 == 0 {
                    game.push_ship(self.builder1.build(), 0, 0);
                    game.push_ship(self.builder2.build(), 1, 0);
                }
            }
            game.tick();
            self.ticks += 1;
        }
    }
    pub fn spawn_ship(&mut self, player: usize, lane: usize, builder_id: usize) -> Result<(), ()> {
        unimplemented!()
    }
}
