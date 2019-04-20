use crate::hex::Hex;

const _EVEN: i8 = 1;
const ODD: i8 = -1;
pub const OFFSET: i8 = ODD;

#[derive(Debug, Clone, Copy)]
pub struct OffsetCoord {
    pub col: i8,
    pub row: i8,
}

impl From<Hex> for OffsetCoord {
    fn from(hex: Hex) -> Self {
        let col = hex.q() + (hex.r() + OFFSET * (hex.r() & 1) / 2);
        let row = hex.r();

        Self { col, row }
    }
}
