extern crate common;

use common::game::Game;
use common::game::ship::base_ship::BaseShip;

#[test]
fn create_push_tick() {
    let mut g = Game::new(6, 50);
    g.push_ship(BaseShip::new(), 0, 0);
    for _ in 0..100 {
        g.tick();
    }
}
