use std::collections::HashSet;
use std::f32;

use ggez::graphics::{self, Point, Color, DrawMode};
use ggez::Context;
use ggez::GameResult;

use terrain::Terrain;
use unit::Unit;
use feature::Feature;

pub const SIZE: u32 = 50;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Hex {
    pub features: Option<HashSet<Feature>>,
    pub position: (i8, i8),
    #[serde(default)]
    pub terrain: Terrain,
    pub unit: Option<Unit>,
    pub victory_point: Option<VictoryPoint>,
}

impl Hex {
    pub fn new(position: (i8, i8)) -> Self {
        Hex {
            position: position,
            ..Self::default()
        }
    }

    fn pixel_position(&self) -> (f32, f32) {
        let size = SIZE as f32;
        let q = self.position.0 as f32;
        let r = self.position.1 as f32;

        (size * 3f32.sqrt() * (q + r/2.), size * 3./2. * r)
    }

    fn corners(&self) -> ([Point; 6]) {
        let size = SIZE as f32;
        let mut corners = [Point::zero(); 6];

        let (center_x, center_y) = self.pixel_position();

        for (i, val) in (0..6).enumerate()
        {
            let angle_deg = 60. * val as f32 + 30.;
            let angle_rad = f32::consts::PI / 180. * angle_deg;
            let x = center_x + size * f32::cos(angle_rad);
            let y = center_y + size * f32::sin(angle_rad);

            corners[i] = Point::new(x + size, y + size);

        }

        corners
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {

        let color = match self.terrain {
            Terrain::Plain => Color::from((2, 255, 50)),
            Terrain::Forest => Color::from((21, 51, 19)),
            Terrain::River => Color::from((5, 102, 141)),
            Terrain::Town => Color::from((79, 98, 114)),
        };

        let points = self.corners();

        graphics::set_color(ctx, color)?;
        graphics::polygon(ctx, DrawMode::Fill, &points)?;
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum VictoryPoint {
    HoldPoint,
}
