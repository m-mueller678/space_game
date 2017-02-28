use super::game::ship::BaseShipBuilder;

mod buf_stream;

pub use self::buf_stream::BufStream;

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientJoin {
    Create,
    //JoinFail | Created(id)->Start
    Join(u32),
    //JoinFail | Start
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerJoin {
    Created(u32),
    Start(usize),
    JoinFail
}


#[cfg_attr(feature = "graphics", derive(Serialize))]
#[derive(Deserialize, Debug)]
//exchanged by clients through server after receiving ServerJoin::Start
pub struct ClientStart {
    pub ships: Vec<BaseShipBuilder>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ClientGame {
    SpawnShip { id: usize, lane: usize },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ServerGame {
    pub tick: usize,
    pub events: Vec<(usize, ServerEvent)>
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ServerEvent {
    SpawnShip {
        player: usize,
        lane: usize,
        id: usize,
    }
}
