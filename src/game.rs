use std::fs::File;
use std::time::Duration;

use ggez::event::{self, MouseButton};
use ggez::graphics::{self, Text};
use ggez::{Context, GameResult};
use ggez::graphics::Point2 as Point;
use merge::merge_keys_serde as merge_keys;
use rand::{self, Rng};
use yaml;

use crate::{
    map::Map,
    player::Player,
};


use super::{WIDTH, HEIGHT};

pub struct Game {
    map: Map,
    player: Player,
}

impl Game {
    pub fn new(_ctx: &mut Context) -> GameResult<Self> {
        Ok(Game {
            map: Map::new(13, 9),
            player: Player::default(),
            /*map: {
                let f = File::open("./resources/maps/plains/pegasus_bridge.yml")?;
                let value = merge_keys(yaml::from_reader(f).unwrap()).unwrap();

                Map::from_data(yaml::from_value(value).unwrap())
            },*/
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
        self.map.draw(ctx, &self.player)?;
        graphics::present(ctx);

        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, button: MouseButton, x: i32, y: i32) {
        let hex = self.map.layout.pixel_to_hex(Point::new(x as f32, y as f32)).round();

        if self.map.grid.contains_key(&hex) {
            self.player.select(hex);
        } else {
            self.player.deselect();
        }
    }
}
