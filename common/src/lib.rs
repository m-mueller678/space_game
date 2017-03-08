#[macro_use]
extern crate serde_derive;
pub extern crate serde;
pub extern crate serde_json;
#[macro_use]
extern crate log;

pub mod game;
#[cfg(feature = "graphics")]
mod graphics;

#[cfg(not(feature = "graphics"))]
mod graphics {
    pub type Sprite = ();
    pub type CompositeTexture = ();
}

#[cfg(feature = "protocol")]
pub mod protocol;

