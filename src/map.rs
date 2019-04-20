use std::collections::HashMap;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Color;

use crate::hex::Hex;
use crate::layout::Layout;
use crate::tile::Tile;
use crate::player::Player;

pub struct Map {
    pub grid: HashMap<Hex, Tile>,
    pub layout: Layout,
}

impl Map {
    pub fn new(width: i8, height: i8) -> Self {
        let mut grid = HashMap::with_capacity((width * height) as usize);

        for r in 0..height {
            let r_offset = r >> 1;
            for q in -r_offset..(width - r_offset) {
                grid.insert(Hex::new(q, r, -q-r), Tile::new());
            }
        }

        Self { grid, layout: Layout::default() }
    }

    pub fn draw(&mut self, ctx: &mut Context, player: &Player) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from((0, 0, 0)));

        for (hex, tile) in self.grid.iter_mut() {
            self.layout.draw_hex(ctx, *hex, tile)?;
        }

        if let Some(hex) = player.selected() {
            self.layout.draw_selection(ctx, hex, &self.grid[&hex])?;

            for (target, tile) in self.grid.iter_mut() {
                self.layout.draw_distance(ctx, hex, *target, tile)?;
            }
        }

        Ok(())
    }
}
