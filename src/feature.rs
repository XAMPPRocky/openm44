use ggez::{Context, GameResult};
use ggez::graphics;

use hex::{Hex, Direction};
use self::Feature::*;

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Feature {
    Bridge,
    Sandbags,
    BarbedWire,
}

impl Feature {
    pub fn draw(&self, hex: &Hex, ctx: &mut Context) -> GameResult<()> {
        let original = graphics::get_line_width(ctx);
        match *self {
            Sandbags => {
                let unit = hex.unit.unwrap();
                let mut arc = hex.arc(Direction::South);
                for point in &mut arc {
                    point.y -= 5.;
                }
                graphics::set_color(ctx, unit.faction.colour())?;
                graphics::set_line_width(ctx, 5.);
                graphics::line(ctx, &arc)?;
            }

            _ => {}
        }

        graphics::set_line_width(ctx, original);

        Ok(())
    }
}
