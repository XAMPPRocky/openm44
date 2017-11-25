use std::fmt;

use ggez::graphics::Color;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Faction {
    Allies,
    Axis,
}

impl Faction {
    pub fn colour(&self) -> Color {
        match *self {
            Faction::Allies => Color::from((255, 255, 255)),
            Faction::Axis => Color::from((0, 0, 0)),
        }
    }
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let faction = match *self {
            Faction::Allies => "A",
            Faction::Axis => "X",
        };

        f.write_str(faction)
    }
}
