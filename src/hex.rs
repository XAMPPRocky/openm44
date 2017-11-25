use std::collections::HashSet;
use std::f32;

use ggez::graphics::{self, Point, Color, DrawMode};
use ggez::Context;
use ggez::GameResult;

use terrain::Terrain;
use unit::Unit;
use feature::Feature;

pub const SIZE: u32 = 50;
pub const OFFSET: f32 = SIZE as f32;

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
        let q = self.position.0 as f32;
        let r = self.position.1 as f32;

        (OFFSET * 3f32.sqrt() * (q + r/2.), OFFSET * 3./2. * r)
    }

    fn corners(&self) -> ([Point; 6]) {
        let mut corners = [Point::zero(); 6];
        let north = self.arc(Direction::North);
        let south = self.arc(Direction::South);

        for (i, val) in north.into_iter().chain(south.into_iter()).enumerate() {
            corners[i] = *val;
        }

        corners
    }

    fn arc(&self, direction: Direction) -> ([Point; 3]) {
        let mut arc = [Point::zero(); 3];
        let (center_x, center_y) = self.pixel_position();

        for (i, val) in direction.degrees().into_iter().enumerate() {
            let angle = val.to_radians();
            let x = center_x + OFFSET * f32::cos(angle);
            let y = center_y + OFFSET * f32::sin(angle);

            arc[i] = Point::new(x + OFFSET, y + OFFSET);
        }

        arc
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

        if let Some(unit) = self.unit {
            unit.draw(self.pixel_position(), ctx)?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum VictoryPoint {
    HoldPoint,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Direction {
    North,
    South,
}

impl Direction {
    fn degrees(&self) -> [f32; 3] {
        match *self {
            Direction::North => [210., 270., 330.],
            Direction::South => [30., 90., 150.],
        }
    }
}
