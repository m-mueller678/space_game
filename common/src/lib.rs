#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate serde_json;

#[cfg(feature = "graphics")]
extern crate sfml;
#[cfg(feature = "graphics")]
extern crate rand;
#[cfg(feature = "graphics")]
#[macro_use]
extern crate lazy_static;

pub mod game;
mod graphics;

#[cfg(feature = "graphics")]
pub use graphics::init_thread_texture_path;
