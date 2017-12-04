use rand::{Rand, Rng};

use self::Dice::*;

#[derive(Clone, Copy, Debug)]
pub enum Dice {
    Infantry,
    Armor,
    Grenade,
    Flag,
    Star,
}

impl Rand for Dice {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let result: u32 = rng.gen_range(1, 6);

        match result {
            1 => Star,
            2 => Flag,
            3 => Grenade,
            4 => Armor,
            5 | 6 => Infantry,
            _ => unreachable!(),
        }
    }
}
