use std::collections::HashSet;

use ggez::{Context, GameResult};
use ggez::graphics::{self, DrawMode, Rect};
use serde::{Deserialize, Deserializer};

use hex::{Hex, Direction, OFFSET, X_OFFSET};
use self::Feature::*;
use terrain::Terrain;
use unit::UnitType;

#[derive(Clone, Debug, Default)]
pub struct Features(Option<HashSet<Feature>>);

impl<'de> Deserialize<'de> for Features {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        Deserialize::deserialize(deserializer).map(Features)
    }
}

macro_rules! unwrap {
    ($this:ident, $else:expr) => {
        if $this.0.is_some() {
            $this.0.as_ref().unwrap()
        } else {
            $else
        }
    }
}

impl Features {
    pub fn protection(&self) -> u8 {
        unwrap!(self, {return 0}).iter().map(|f| f.protection()).max().unwrap_or(0)
    }

    pub fn contains(&self, feature: &Feature) -> bool {
        unwrap!(self, {return false}).contains(feature)
    }

    pub fn remove(&mut self, feature: &Feature) {
        if let Some(features) = self.0.as_mut() {
            features.remove(feature);
        }
    }

    pub fn stops_movement(&self, unit_type: UnitType) -> bool {
        match unit_type {
            UnitType::Infantry => self.contains(&Feature::BarbedWire),
            _ => false,
        }
    }

    pub fn draw(&self, hex: &Hex, ctx: &mut Context) -> GameResult<()> {
        if let Some(features) = self.0.as_ref() {
            for feature in features {
                feature.draw(hex, ctx)?;
            }
        }

        Ok(())
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum Feature {
    Bridge,
    Sandbags,
    BarbedWire,
}

impl Feature {
    pub fn draw(&self, hex: &Hex, ctx: &mut Context) -> GameResult<()> {
        match *self {
            Sandbags => {
                let unit = hex.unit.unwrap();
                let mut arc = hex.arc(Direction::South);
                for point in &mut arc {
                    point.y -= 5.;
                }
                graphics::set_color(ctx, unit.faction.colour())?;
                graphics::line(ctx, &arc, 5.)?;
            }

            Bridge => {
                graphics::set_color(ctx, Terrain::Town.colour().into())?;
                let (x, y) = hex.pixel_position();
                let (width, height) = (X_OFFSET * 2., OFFSET / 1.5);
                let rect = Rect::new(x - width / 2., y - height / 2., width, height);

                graphics::rectangle(ctx, DrawMode::Fill, rect)?;
            }

            _ => {}
        }

        Ok(())
    }

    pub fn protection(&self) -> u8 {
        match *self {
            Bridge => 0,
            Sandbags => 1,
            BarbedWire => 0,
        }
    }
}
