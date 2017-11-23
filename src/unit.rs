use std::fmt;

use serde::de::{self, Deserialize, Deserializer, Visitor, MapAccess};

use faction::Faction;
use self::UnitType::*;

#[derive(Clone, Copy, Debug)]
pub struct Unit {
    pub unit_type: UnitType,
    pub current_health: u8,
    pub faction: Faction,
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
