use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::ops::{Deref, DerefMut};

use ggez::GameResult;
use ggez::Context;
use ggez::graphics::{self, Color};
use ggez::timer;

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

    pub fn change_hexs<F>(&mut self, positions: ((i8, i8), (i8, i8)), fun: F)
        where F: FnOnce(&mut Self, &mut Hex, &mut Hex)
    {
        let mut a = self.map.get(&positions.0).unwrap().clone();
        let mut b = self.map.get(&positions.1).unwrap().clone();

        fun(self, &mut a, &mut b);

        self.map.insert(positions.0, a);
        self.map.insert(positions.1, b);

    }

    pub fn hex_distance(src: (i8, i8), dest: (i8, i8)) -> u8 {
        let qdiff = i8::abs(src.0 - dest.0);
        let rdiff = i8::abs(src.1 - dest.1);
        let totaldiff = i8::abs(src.0 + src.1 - dest.0 - dest.1);

        ((qdiff + totaldiff + rdiff) / 2) as u8
    }

    pub fn can_reach(&self, limit: u8, src: &(i8, i8)) -> HashSet<(i8, i8)>
    {
        let mut visited = HashSet::new();
        visited.insert(*src);
        let mut fringes = Vec::new();
        fringes.push(vec![*src]);

        for k in 1..limit + 1 {
            fringes.push(vec![]);
            for cube in fringes[(k - 1) as usize].clone() {
                for neighbour in self.map[&cube].neighbours().iter().filter(|n| self.map.contains_key(n)) {
                    let neighbour = &self.map[&neighbour];
                    if !visited.contains(&neighbour.position) && neighbour.blocks_movement() {
                        visited.insert(neighbour.position);
                        fringes[k as usize].push(neighbour.position);
                    }
                }
            }
        }

        visited
    }

    pub fn reset_view(&mut self) {
        for hex in self.map.values_mut()
            .filter(|h| (h.selected || h.distance.is_some()))
            {
                hex.selected = false;
                hex.distance = None;
            }
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from((0, 0, 0)));

        for (_, hex) in &self.map {
            hex.draw(ctx)?;
        }

        timer::sleep(Duration::new(0, 0));
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
