use std::collections::HashSet;
use std::f32;

use ggez::graphics::{self, Point, Color, DrawMode, Text};
use ggez::Context;
use ggez::GameResult;

use terrain::Terrain;
use unit::Unit;
use feature::Feature;
use hsl::Hsl;
use unit::FONT;

pub const SIZE: u32 = 50;
pub const OFFSET: f32 = SIZE as f32;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Hex {
    pub features: Option<HashSet<Feature>>,
    pub position: (i8, i8),
    #[serde(default)]
    pub selected: bool,
    #[serde(default)]
    pub terrain: Terrain,
    pub unit: Option<Unit>,
    pub victory_point: Option<VictoryPoint>,
    #[serde(default)]
    pub distance: Option<u8>,
}

impl Hex {
    pub fn new(position: (i8, i8)) -> Self {
        Hex {
            position: position,
            ..Self::default()
        }
    }

    pub fn remove_unit(&mut self) -> Option<Unit> {
        let wrapped = self.unit.take();

        if let Some(unit) = wrapped {
            if let Some(features) = self.features.as_mut() {
                features.remove(&Feature::Sandbags);
            }
        }

        wrapped
    }

    pub fn blocks_movement(&self) -> bool {
        self.terrain.is_walkable() && self.unit.is_none()
    }

    pub fn can_fire_at(&self, dest: &Hex) -> bool {
        if self.unit.is_none() || dest.unit.is_none() {
            return false
        }

        let src_unit = self.unit.unwrap();
        let dest_unit = dest.unit.unwrap();

        /*
        if dest.distance_from(self) > src_unit.range() {
            return false
        }
        */

        true
    }

    pub fn neighbours(&self) -> [(i8, i8); 6] {
        let q = self.position.0;
        let r = self.position.1;

        [
            (q, r - 1),
            (q + 1, r - 1),
            (q + 1, r),
            (q, r + 1),
            (q - 1, r + 1),
            (q - 1, r),
        ]
    }

    fn pixel_position(&self) -> (f32, f32) {
        let q = self.position.0 as f32;
        let r = self.position.1 as f32;

        (OFFSET * 3f32.sqrt() * (q + r/2.), OFFSET * 3./2. * r)
    }

    fn corners(&self) -> [Point; 6] {
        let mut corners = [Point::zero(); 6];
        let north = self.arc(Direction::North);
        let south = self.arc(Direction::South);

        for (i, val) in north.into_iter().chain(south.into_iter()).enumerate() {
            corners[i] = *val;
        }

        corners
    }

    pub fn arc(&self, direction: Direction) -> ([Point; 3]) {
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

        let points = self.corners();

        graphics::set_color(ctx, self.terrain.colour().into())?;
        graphics::polygon(ctx, DrawMode::Fill, &points)?;

        if let Some(unit) = self.unit.clone() {
            unit.draw(self.pixel_position(), ctx)?;
        }

        if let Some(ref features) = self.features {
            for feature in features {
                feature.draw(self, ctx);
            }
        }

        if self.selected {
            graphics::set_color(ctx, Color::from((252, 246, 177)))?;
            graphics::polygon(ctx, DrawMode::Line, &points)?;
        }

        if let Some(distance) = self.distance {
            let (x, y) = self.pixel_position();
            let x = x + OFFSET;
            let y = y + OFFSET;
            graphics::set_color(ctx, Hsl::new(0., 1., 0.92).into())?;
            let text = Text::new(ctx, &distance.to_string(), &FONT)?;
            graphics::draw(ctx, &text, Point::new(x, y), 0.)?;
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
