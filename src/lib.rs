#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate ggez;
extern crate rand;
extern crate serde;
extern crate serde_yaml as yaml;
extern crate yaml_merge_keys as merge;

pub mod card;
pub mod cube;
pub mod dice;
pub mod faction;
pub mod feature;
pub mod game;
pub mod hex;
pub mod hex2;
pub mod hsl;
pub mod layout;
pub mod map;
pub mod orientation;
pub mod offset;
pub mod terrain;
pub mod turn;
pub mod unit;
pub mod map2;

use hex::SIZE;

pub const HEIGHT: u32 = SIZE * 14;
pub const WIDTH: u32 = (SIZE * 24);


pub fn start() {
}
