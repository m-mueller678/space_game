#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate serde_json;

#[cfg(feature = "graphics")]
extern crate sfml;

pub mod game;
mod graphics;

#[cfg(feature = "protocol")]
pub mod protocol;

#[cfg(feature = "graphics")]
pub use graphics::init_thread_texture_path;
