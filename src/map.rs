use std::collections::HashMap;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics::{self, Color};

use hex::Hex;

pub type Grid = HashMap<(i8, i8), Hex>;

#[derive(Clone, Debug, Deserialize)]
pub struct MapData {
    pub name: String,
    pub biome: Biome,
    map: Vec<Hex>,
}

#[derive(Clone, Debug)]
pub struct Map {
    pub name: String,
    pub biome: Biome,
    pub map: Grid,
}

impl Map {
    pub fn from_data(map_data: MapData) -> Self {
        let mut map = generate_hex_map(13, 9);

        for hex in map_data.map {
            map.insert(hex.position, hex);
        }

        Map {
            name: map_data.name,
            biome: map_data.biome,
            map: map
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from((0, 0, 0)));

        for (_, hex) in self.map.iter()/*.filter(|&(_, h)| h.position.1 == 0)*/ {
            hex.draw(ctx)?;
        }

        Ok(())
    }
}

fn generate_hex_map(width: i8, height: i8) -> Grid {
    let mut map = HashMap::new();
    let mut x_offset = 0;

    for row in 0..height {
        if row % 2 == 0 && row != 0 {
            x_offset -= 1;
        }

        for col in x_offset..(width + x_offset) {
            map.insert((col, row), Hex::new((col, row)));
        }
    }

    map
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Biome {
    Plains
}
