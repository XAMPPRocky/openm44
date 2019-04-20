use ggez::graphics::Text;

#[derive(Default, Clone)]
pub struct Tile {
    pub highlighted: bool,
    pub text: Option<Text>
}

impl Tile {
    pub fn new() -> Self {
        Self::default()
    }
}
