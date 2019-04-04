use rand::{self, Rng};
use serde::de::{self, Deserialize, Deserializer, Unexpected};

use unit::UnitType;

#[derive(Clone, Debug, Default, Deserialize)]
pub struct Deck {
    name: String,
    #[serde(deserialize_with = "deserialise_cards")]
    cards: Vec<Card>,
}

impl Deck {
    pub fn shuffle(&mut self) {
        let mut rng = rand::thread_rng();
        self.cards.sort_by(|_, _| rng.gen::<u64>().cmp(&rng.gen()))
    }
}

#[derive(Clone, Debug, Deserialize)]
struct CardBuilder {
    section: Section,
    units: Units,
    amount: u64,
}

#[derive(Clone, Debug)]
pub enum Card {
    Normal {
        section: Section,
        units: Units,
    },

    Recon(Section),
}

impl Card {
    fn new(builder: &CardBuilder) -> Self {
        Card::Normal {
            section: builder.section,
            units: builder.units,
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub enum Section {
    Left,
    Right,
    Centre,
}

#[derive(Clone, Copy, Debug, Deserialize)]
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

fn deserialise_cards<'de, D>(deserializer: D) -> Result<Vec<Card>, D::Error>
    where D: Deserializer<'de>
{
    let card_builders = Vec::<CardBuilder>::deserialize(deserializer)?;
    let mut cards = Vec::with_capacity(60);

    for card_builder in card_builders {
        for _ in 0..card_builder.amount {
            cards.push(Card::new(&card_builder));
        }
    }

    Ok(cards)
}
