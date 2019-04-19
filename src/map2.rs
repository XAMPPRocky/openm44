use std::collections::HashSet;

use ggez::GameResult;
use ggez::Context;
use ggez::graphics;
use ggez::graphics::Color;

use crate::hex2::Hex;
use crate::layout::Layout;

pub struct Map {
    pub grid: HashSet<Hex>,
    layout: Layout,
}

impl Map {
    pub fn new(width: i8, height: i8) -> Self {
        let mut grid = HashSet::with_capacity((width * height) as usize);

        for r in 0..height {
            let r_offset = r >> 1;
            for q in -r_offset..(width - r_offset) {
                grid.insert(Hex::new(q, r, -q-r));
            }
        }

        /*
        for s in 0..height {
            let offset = s as isize >> 1;
            let s = s as i8;
            for q in (-offset)..(width as isize - offset) {
                let q = q as i8;
                let r = s + q.abs();
                println!("({}, {}, {})", q, r, s);
                grid.insert(Hex::new(q, r, s));
            }
        }
        */

        Self { grid, layout: Layout::default() }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::set_background_color(ctx, Color::from((0, 0, 0)));

        for hex in &self.grid {
            self.layout.draw_hex(ctx, *hex)?;
        }

        Ok(())
    }
}
