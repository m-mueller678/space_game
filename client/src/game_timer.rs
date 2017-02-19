use sfml::system::Clock;
use common::game::ship::BaseShipBuilder;
use common::game::Game;

pub struct GameTimer {
    clock: Clock,
    ticks: u32,
    builder: BaseShipBuilder,
}

impl GameTimer {
    pub fn new(builder: BaseShipBuilder) -> Self {
        GameTimer {
            clock: Clock::new(),
            ticks: 0,
            builder: builder,
        }
    }
    pub fn do_ticks(&mut self, game: &mut Game) {
        while self.ticks < self.clock.get_elapsed_time().as_milliseconds() as u32 / 20 {
            if self.ticks % 128 == 0 {
                game.push_ship(self.builder.build(), self.ticks as usize / 128 % 2, 0);
            }
            game.tick();
            self.ticks += 1;
        }
    }
}
