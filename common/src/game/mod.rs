mod ship;
mod lane;

use self::ship::*;
use self::lane::*;

struct Game {
    lanes: [Vec<Lane>; 2],
}

impl Game {}