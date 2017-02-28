#[macro_use]
extern crate serde_derive;
pub extern crate serde;
pub extern crate serde_json;
#[macro_use]
extern crate log;

#[cfg(feature = "graphics")]
extern crate sfml;

pub mod game;
mod graphics;

#[cfg(feature = "protocol")]
pub mod protocol;

#[cfg(feature = "graphics")]
pub use graphics::init_thread_texture_path;
