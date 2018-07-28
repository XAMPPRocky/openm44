use self::TurnPhase::*;

#[derive(Clone, Copy, Debug)]
pub enum TurnPhase {
    Card,
    Order,
    Move,
    Battle,
    Draw,
}

impl TurnPhase {
    pub fn advance(&mut self) {
        *self = match *self {
            Card => Order,
            Order => Move,
            Move => Battle,
            Battle => Draw,
            Draw => Card,
        }
    }
}

impl Default for TurnPhase {
    fn default() -> Self {
        Card
    }
}
