#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate serde;
extern crate serde_yaml as yaml;
extern crate yaml_merge_keys as merge;
extern crate ggez;

pub mod faction;
pub mod feature;
pub mod hex;
pub mod map;
pub mod terrain;
pub mod unit;

use std::fs::File;
use std::time::Duration;

use map::{Map, MapData};
use hex::SIZE;
use yaml::Value;

use ggez::conf;
use ggez::event;
use ggez::{Context, GameResult};
use ggez::graphics;

const HEIGHT: u32 = SIZE * 14;
const WIDTH: u32 = (SIZE * 24);

struct MainState {
    map: Map
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<Self> {
        Ok(MainState {
            map: {
                let file = File::open("./maps/plains/pegasus_bridge.yml")?;
                let value: Value = yaml::from_reader(file).unwrap();
                let merged = merge::merge_keys_serde(value).unwrap();
                let data: MapData = yaml::from_value(merged).unwrap();

                Map::from_data(data)
            }
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context, _dt: Duration) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);

        self.map.draw(ctx)?;
        graphics::present(ctx);

        Ok(())
    }
}

fn main() {
    run().unwrap();
}

fn run() -> GameResult<()> {
    let mut c = conf::Conf::new();
    c.window_width = WIDTH;
    c.window_height = HEIGHT;
    c.vsync = true;

    let mut ctx = Context::load_from_conf("helloworld", "ggez", c).unwrap();
    let mut state = MainState::new(&mut ctx).unwrap();

    event::run(&mut ctx, &mut state)?;

    Ok(())
}
