use std::fmt;
use std::ops::Deref;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use ggez::graphics::{self, DrawMode, Font, Point, Rect, Text};
use ggez::Context;
use ggez::GameResult;

use faction::Faction;
use self::UnitType::*;
use hex::{Hex, OFFSET};

const BOX_WIDTH: f32 = 80.;
const BOX_HEIGHT: f32 = 20.;

lazy_static! {
    pub static ref FONT: Font = Font::default_font().unwrap();
}

#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub unit_type: UnitType,
    pub current_health: u8,
    pub faction: Faction,
}

impl Unit {
    pub fn fire_at(&self, hex: &mut Hex) {
    }

    pub fn draw(&self, (x, y): (f32, f32), ctx: &mut Context) -> GameResult<()> {
        let x = x + OFFSET;
        let y = y + OFFSET;
        let unit_box = Rect::new(x, y, BOX_WIDTH, BOX_HEIGHT);

        graphics::set_color(ctx, self.faction.colour())?;
        graphics::rectangle(ctx, DrawMode::Fill, unit_box)?;
        let invert = {
            let mut color = graphics::get_color(ctx);
            color.r = 1. - color.r;
            color.g = 1. - color.g;
            color.b = 1. - color.b;
            color
        };

        graphics::set_color(ctx, invert)?;
        let text = Text::new(ctx, &self.to_string(), &FONT)?;
        graphics::draw(ctx, &text, Point::new(x, y), 0.)?;

        Ok(())
    }
}

impl Deref for Unit {
    type Target = UnitType;

    fn deref(&self) -> &Self::Target {
        &self.unit_type
    }
}

impl fmt::Display for Unit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}; {}: {}", self.faction, self.unit_type, self.current_health)
    }
}

impl<'de> Deserialize<'de> for Unit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where D: Deserializer<'de>
    {
        #[derive(Deserialize)]
        #[serde(field_identifier, rename_all = "snake_case")]
        enum Field { Type, Faction, }

        struct UnitVisitor;

        impl<'de> Visitor<'de> for UnitVisitor {
            type Value = Unit;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct Unit")
            }

            fn visit_map<V>(self, mut map: V) -> Result<Unit, V::Error>
                where V: MapAccess<'de>
            {
                let mut _type = None;
                let mut faction = None;

                while let Some(key) = map.next_key()? {
                    match key {
                        Field::Type => {
                            if _type.is_some() {
                                return Err(de::Error::duplicate_field("type"));
                            }
                            _type = Some(map.next_value()?);
                        }
                        Field::Faction => {
                            if faction.is_some() {
                                return Err(de::Error::duplicate_field("faction"));
                            }
                            faction = Some(map.next_value()?);
                        }
                    }
                }

                let _type: UnitType = _type.ok_or_else(|| de::Error::missing_field("type"))?;
                let faction = faction.ok_or_else(|| de::Error::missing_field("faction"))?;

                Ok(Unit {
                    current_health: _type.health(),
                    unit_type: _type,
                    faction: faction,

                })
            }
        }

        const FIELDS: &'static [&'static str] = &["type", "faction"];
        deserializer.deserialize_struct("Unit", FIELDS, UnitVisitor)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum UnitType {
    Infantry,
    Armor,
    Artillery,
}

impl UnitType {
    pub fn movement(&self) -> u8 {
        match *self {
            Armor => 3,
            Artillery => 1,
            Infantry => 2,
        }
    }

    /// How far a unit can move and still be able to attack.
    pub fn movement_threshold(&self) -> u8 {
        match *self {
            Armor => 3,
            Artillery => 0,
            Infantry => 1,
        }
    }

    pub fn range(&self) -> u8 {
        match *self {
            Armor | Infantry => 3,
            Artillery => 6,
        }
    }

    // [(range_threshold, damage)]
    pub fn range_effectiveness(&self) -> &'static [(u8, u8)] {
        match *self {
            Armor => &[(1, 3)],
            Artillery => &[(1, 3), (3, 2), (5, 1)],
            Infantry => &[(1, 3), (2, 2), (3, 1)],
        }
    }

    pub fn health(&self) -> u8 {
        match *self {
            Armor => 3,
            Artillery => 2,
            Infantry => 4,
        }
    }
}

impl fmt::Display for UnitType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let unit = match *self {
            Armor => "Arm",
            Artillery => "Art",
            Infantry => "Inf",
        };

        f.write_str(unit)
    }
}
