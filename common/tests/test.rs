extern crate common;

use common::*;

const BUILDER_JSON: &'static str = r###"{
        "laser_dmg_mult":2000000000,
        "plasma_dmg_mult":2000000000,
        "accel":1,
        "max_speed":20,
        "max_health":1000,
        "weapons":[
            {"range":1000,"offset":[0,0],"priority":20,"class":{"Laser":{"color":[0,0,0],"power":20}}}
        ],
        "texture":{
            "parts":[
                {"texture":0,"rect":[[-40,-10],[40,10]]}
            ]
        }
    }"###;

#[test]
fn create_push_tick() {
    let builder: game::ship::BaseShipBuilder = serde_json::from_str(BUILDER_JSON).unwrap();
    let mut g = game::Game::new(6, 50);
    g.push_ship(builder.build(), 0, 0);
    g.push_ship(builder.build(), 1, 0);
    for _ in 0..10000 {
        g.tick();
    }
}