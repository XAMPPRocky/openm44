use self::Terrain::*;
use hsl::Hsl;

#[derive(Clone, Copy, Debug, Deserialize, Serialize)]
pub enum Terrain {
    Plain,
    Forest,
    River,
    Town
}

impl Terrain {
    pub fn is_walkable(&self) -> bool {
        match *self {
            Forest | Plain | Town => true,
            River => false,
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
}

impl Default for Terrain {
    fn default() -> Self {
        Terrain::Plain
    }
}
