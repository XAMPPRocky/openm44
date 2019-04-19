use std::fs::File;
use std::time::Duration;

use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Text};
use ggez::{Context, GameResult};
use merge::merge_keys_serde as merge_keys;
use rand::{self, Rng};
use yaml;

use crate::{
    card::{Card, Deck},
    cube::Cube,
    hex::{Coordinate, OFFSET, X_OFFSET},
    map2::Map,
    turn::TurnPhase,
    unit::FONT,
};


use super::{WIDTH, HEIGHT};

pub struct Game {
    map: Map,
    src_position: Option<Coordinate>,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            map: Map::new(13, 9),
            /*map: {
                let f = File::open("./resources/maps/plains/pegasus_bridge.yml")?;
                let value = merge_keys(yaml::from_reader(f).unwrap()).unwrap();

                Map::from_data(yaml::from_value(value).unwrap())
            },*/
            src_position: None,
        })
    }
}

impl event::EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_resolution(ctx, WIDTH, HEIGHT);
        graphics::clear(ctx);
        self.map.draw(ctx)?;
        graphics::present(ctx);

        Ok(())
    }

}
