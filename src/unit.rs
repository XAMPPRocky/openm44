use std::fmt;
use std::ops::Deref;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};
use ggez::graphics::{self, DrawMode, Font, Point2 as Point, Rect, Text};
use ggez::Context;
use ggez::GameResult;

use faction::Faction;
use self::UnitType::*;
use hex::{Hex, OFFSET};
use dice::Dice;

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
    pub moved: u8,
    pub destroy: bool,
}

impl Unit {
    pub fn draw(&self, (x, y): (f32, f32), ctx: &mut Context) -> GameResult<()> {
        let (x, y) = (x - (BOX_WIDTH / 2.), y - (BOX_HEIGHT / 2.));
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

        let mut text = Text::new(ctx, &self.to_string(), &FONT)?;
        text.set_filter(graphics::FilterMode::Nearest);

        let x = if (text.width() as f32) < BOX_WIDTH {
            let difference = BOX_WIDTH - text.width() as f32;
            x + (difference / 2.)
        } else {
            x
        };

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
                    moved: 0,
                    destroy: false,
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

macro_rules! is {
    ($(($fn:ident: $item:ident),)*) => {
        $(
            fn $fn(&self) -> bool {
                match *self {
                    $item => true,
                    _ => false,
                }
            }
        )*
    }
}

impl UnitType {
    pub fn movement(&self) -> u8 {
        match *self {
            Armor => 3,
            Artillery => 1,
            Infantry => 2,
        }
    }

    is! {
        (is_infantry: Infantry),
        (is_armor: Armor),
        (is_artillery: Artillery),
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
    pub fn range_effectiveness(&self) -> &'static [u8] {
        match *self {
            Armor => &[3, 3, 3],
            Artillery => &[3, 3, 2, 2, 1, 1],
            Infantry => &[3, 2, 1],
        }
    }

    pub fn health(&self) -> u8 {
        match *self {
            Armor => 3,
            Artillery => 2,
            Infantry => 4,
        }
    }

    pub fn takes_causality(&self, result: Dice) -> bool {
        match result {
            Dice::Armor if self.is_armor() => true,
            Dice::Infantry if self.is_infantry() => true,
            Dice::Grenade => true,
            _ => false,
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
