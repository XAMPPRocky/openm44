use rand::{self, Rng};
use serde::de::{self, Deserialize, Deserializer, Unexpected};

use unit::UnitType;

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Card {
    Normal {
        section: Section,
        units: Units,
    },

    Recon(Section),
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Deck {
    name: String,
    cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.sort_by(|_, _| rng.gen::<u64>().cmp(&rng.gen()))
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Section {
    Left,
    Right,
    Centre,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(untagged)]
pub enum Units {
    #[serde(deserialize_with = "literal_all")]
    All,
    UnitType(UnitType),
    Limited(u8),
}

fn literal_all<'de, D>(deserializer: D) -> Result<(), D::Error>
    where D: Deserializer<'de>
{
    let s = String::deserialize(deserializer)?;
    if s == "All" {
        Ok(())
    } else {
        Err(de::Error::invalid_value(Unexpected::Str(&s), &"the string 'All'"))
    }
}
