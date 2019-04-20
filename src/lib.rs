#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate ggez;
extern crate rand;
extern crate serde;
extern crate serde_yaml as yaml;
extern crate yaml_merge_keys as merge;

pub mod game;
pub mod hex;
pub mod hsl;
pub mod layout;
pub mod map;
pub mod offset;
pub mod player;
pub mod tile;
pub mod orientation;

pub use self::layout::{HEIGHT, WIDTH};

pub fn start() {
}
