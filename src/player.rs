use crate::hex::Hex;

#[derive(Clone, Default)]
pub struct Player {
    selected_hex: Option<Hex>,
}

impl Player {
    pub fn selected(&self) -> Option<Hex> {
        self.selected_hex
    }

    pub fn select(&mut self, hex: Hex) -> Option<Hex> {
        self.selected_hex.replace(hex)
    }

    pub fn deselect(&mut self) -> Option<Hex> {
        self.selected_hex.take()
    }

}
