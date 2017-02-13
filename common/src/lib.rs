#[macro_use]
extern crate serde_derive;
extern crate serde;
pub extern crate serde_json;

#[cfg(feature = "graphics")]
extern crate sfml;
#[cfg(feature = "graphics")]
extern crate rand;

pub mod game;
mod graphics;