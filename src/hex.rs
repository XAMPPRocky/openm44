use std::collections::HashSet;
use std::f32;

use ggez::graphics::{self, Point, Color, DrawMode, Text};
use ggez::Context;
use ggez::GameResult;

use terrain::Terrain;
use unit::Unit;
use feature::{Feature, Features};
use hsl::Hsl;
use unit::{FONT, UnitType};

pub const SIZE: u32 = 50;
pub const OFFSET: f32 = SIZE as f32;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Hex {
    pub features: Features,
    pub position: (i8, i8),
    #[serde(default)]
    pub selected: bool,
    #[serde(default)]
    pub terrain: Terrain,
    pub unit: Option<Unit>,
    pub victory_point: Option<VictoryPoint>,
    #[serde(default)]
    pub distance: Option<u8>,
    #[serde(default)]
    pub dice: Option<u8>,
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
            self.features.remove(&Feature::Sandbags);
        }

        wrapped
    }

    pub fn blocks_sight(&self) -> bool {
        self.terrain.blocks_sight(self.features.clone()) || self.unit.is_some()
    }

    pub fn blocks_movement(&self) -> bool {
        self.terrain.blocks_movement(self.features.clone()) || self.unit.is_some()
    }

    pub fn stops_movement(&self, unit_type: Option<UnitType>) -> bool {
        self.terrain.stops_movement() ||
            (unit_type.is_some() && self.features.stops_movement(unit_type.unwrap()))
    }

    pub fn reduce_dice(&self, unit: UnitType, max: u8) -> u8 {
        let terrain_reduction = max.saturating_sub(self.terrain.protection(unit));
        let feature_reduction = max.saturating_sub(self.features.protection());

        u8::min(terrain_reduction, feature_reduction)
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

    pub fn pixel_position(&self) -> (f32, f32) {
        let q = self.position.0 as f32;
        let r = self.position.1 as f32;

        (
            (OFFSET * 3f32.sqrt() * (q + r/2.)) + OFFSET,
            (OFFSET * 3./2. * r) + OFFSET,
        )
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

            arc[i] = Point::new(x, y);
        }

        arc
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult<()> {
        let points = self.corners();

        graphics::set_color(ctx, self.terrain.colour().into())?;
        graphics::polygon(ctx, DrawMode::Fill, &points)?;

        self.features.draw(self, ctx)?;

        if self.selected {
            graphics::set_color(ctx, Color::from((252, 246, 177)))?;
            graphics::polygon(ctx, DrawMode::Line, &points)?;
        }

        if let Some(unit) = self.unit.clone() {
            unit.draw(self.pixel_position(), ctx)?;
        }

        if let Some(distance) = self.distance {
            let (x, y) = self.pixel_position();

            graphics::set_color(ctx, Hsl::new(0., 1., 0.92).into())?;
            let mut text = Text::new(ctx, &distance.to_string(), &FONT)?;
            text.set_filter(graphics::FilterMode::Nearest);
            graphics::draw(ctx, &text, Point::new(x, y), 0.)?;
        }

        if let Some(dice) = self.dice {
            let (x, y) = self.pixel_position();

            let display = {
                let mut x = String::from("X");

                for _ in 1..dice {
                    x += "X";
                }

                x
            };

            graphics::set_color(ctx, Hsl::new(0., 1., 0.92).into())?;
            let mut text = Text::new(ctx, &display, &FONT)?;
            text.set_filter(graphics::FilterMode::Nearest);
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
