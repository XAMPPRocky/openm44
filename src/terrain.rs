use std::collections::HashSet;

use self::Terrain::*;
use hsl::Hsl;
use feature::{Feature, Features};
use unit::UnitType;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Terrain {
    Plain,
    Forest,
    River,
    Town
}

impl Terrain {
    pub fn blocks_movement(&self, features: Features) -> bool {
        match *self {
            Forest | Plain | Town => false,
            River => !features.contains(&Feature::Bridge),
        }
    }

    pub fn colour(&self) -> Hsl {
        const PLAIN: Hsl = Hsl { hue: 79., saturation: 0.45, lightness: 0.5 };

        match *self {
            Plain => PLAIN,
            Forest => {
                let mut colour = PLAIN;
                colour.darken(0.2);
                colour
            }
            River => Hsl::new(203., 0.48, 0.44),
            Town => Hsl::new(198., 0.21, 0.39),
        }
    }

    pub fn blocks_sight(&self, _features: Features) -> bool {
        match *self {
            Plain | River => false,
            Forest | Town => true
        }
    }

    pub fn stops_movement(&self) -> bool {
        match *self {
            Plain | River => false,
            Forest | Town => true,
        }
    }

    pub fn protection(&self, against: UnitType) -> u8 {
        macro_rules! unit_match {
            ($($unit:ident => $amount:expr,)*) => {{
                use unit::UnitType::*;
                match against {
                    $(
                        $unit => $amount,
                    )*
                    Artillery => 0,
                }
            }}
        }

        match *self {
            Plain => 0,
            Town => unit_match!(Infantry => 1, Armor => 2,),
            Forest => unit_match!(Infantry => 1, Armor => 2,),
            River => 0,
        }
    }
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Plain
    }
}
