use std::collections::{HashMap, HashSet};
use std::ops::{Deref, DerefMut};

use ggez::GameResult;
use ggez::Context;
use ggez::graphics::{self, Color};

use unit::Unit;
use hex2::Hex;
use hex::Hex as Tile;

pub type Grid = HashMap<Hex, Tile>;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct MapData {
    pub name: String,
    pub biome: Biome,
    map: Vec<Tile>,
}

#[derive(Clone, Debug, Default)]
pub struct Map {
    pub name: String,
    pub biome: Biome,
    pub map: Grid,
}

impl Map {
    pub fn from_data(map_data: MapData) -> Self {
        let mut map = Self::generate(13, 9);

        for tile in map_data.map {
            let (q, r) = tile.position;
            let hex = Hex::new(q, r, -q-r);
            map.insert(hex, tile);
        }

        Map {
            name: map_data.name,
            biome: map_data.biome,
            map: map
        }
    }

    fn generate(width: i8, height: i8) -> Grid {
        let mut map = HashMap::new();

        for r in 0..height {
            let r_offset = r >> 1; // or r>>1
            for q in (-r_offset)..(width-r_offset) {
                map.insert(Hex::new(q, r, -q-r), Tile::new((q, r)));
            }
        }

        map
    }

    pub fn change_hexs<F>(&mut self, positions: ((i8, i8), (i8, i8)), fun: F)
        where F: FnOnce(&mut Self, &mut Hex, &mut Hex)
        {
            unimplemented!()

        }

    pub fn can_move_to(&self, limit: u8, src: &(i8, i8)) -> HashSet<(i8, i8)> {
        unimplemented!()
    }

    // TODO: reduce code reuse
    pub fn can_fire_at(&self, unit: Unit, src: &(i8, i8)) -> HashSet<(i8, i8)> {
        unimplemented!()
    }

    pub fn reset_view(&mut self) {
        for hex in self.map.values_mut()
            .filter(|h| (h.selected || h.distance.is_some()) || h.dice.is_some())
        {
            hex.selected = false;
            hex.distance = None;
            hex.dice = None;
            let mut destroy = if let Some(unit) = hex.unit {
                    unit.destroy
            } else {
                false
            };

            if destroy {
                hex.remove_unit();
            }
        }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from((0, 0, 0)));

        for hex in self.map.values() {
            hex.draw(ctx)?;
        }

        /*
        for hex in self.map.values() {
            hex.draw_borders(ctx)?;
        }

        for hex in self.map.values() {
            hex.draw_features(ctx)?;
        }

        for hex in self.map.values().filter(|h| h.has_unit()) {
            hex.draw_units(ctx)?;
        }
        */

        for hex in self.map.values() {
            hex.draw_overlay(ctx)?;
        }

        Ok(())
    }
}

impl Deref for Map {
    type Target = Grid;

    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl DerefMut for Map {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Biome {
    Plains
}

impl Default for Biome {
    fn default() -> Self {
        Biome::Plains
    }
}
