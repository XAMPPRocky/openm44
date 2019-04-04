use std::ops::{Add, Mul, Sub};

const HEX_DIRECTIONS: [Hex; 6] = [
    Hex::new_unchecked(1, 0, -1), Hex::new_unchecked(1, -1, 0), Hex::new_unchecked(0, -1, 1),
    Hex::new_unchecked(-1, 0, 1), Hex::new_unchecked(-1, 1, 0), Hex::new_unchecked(0, 1, -1),
];

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Hex {
    coordinates: [i8; 3],
}

impl Hex {
    pub fn new(q: i8, r: i8, s: i8) -> Self {
        assert_eq!(0, q + r + s);

        Hex { coordinates: [q, r, s] }
    }

    const fn new_unchecked(q: i8, r: i8, s: i8) -> Self {
        Hex { coordinates: [q, r, s] }
    }

    pub fn new_coord(q: i8, r: i8) -> Self {
        Self::new(q, r, -q-r)
    }

    pub fn q(&self) -> i8 {
        self.coordinates[0]
    }

    pub fn r(&self) -> i8 {
        self.coordinates[1]
    }

    pub fn s(&self) -> i8 {
        self.coordinates[2]
    }

    pub fn len(self) -> usize {
        (self.coordinates.iter().fold(0, |acc, i| acc + i.abs()) / 2) as usize
    }

    pub fn distance(self, rhs: Self) -> usize {
        (self - rhs).len()
    }

    pub fn direction(direction: usize) -> Self {
        assert!(0 <= direction && direction < 6);

        HEX_DIRECTIONS[direction]
    }

    pub fn neighbour(self, direction: usize) -> Self {
        self + Self::direction(direction)
    }
}

impl From<[i8; 3]> for Hex {
    fn from(coordinates: [i8; 3]) -> Self {
        Hex { coordinates }
    }
}

impl Add<Hex> for Hex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut coordinates = [0; 3];

        coordinates[0] = self.coordinates[0] + rhs.coordinates[0];
        coordinates[1] = self.coordinates[1] + rhs.coordinates[1];
        coordinates[2] = self.coordinates[2] + rhs.coordinates[2];

        Self::from(coordinates)
    }
}

impl Sub<Hex> for Hex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut coordinates = [0; 3];

        coordinates[0] = self.coordinates[0] - rhs.coordinates[0];
        coordinates[1] = self.coordinates[1] - rhs.coordinates[1];
        coordinates[2] = self.coordinates[2] - rhs.coordinates[2];

        Self::from(coordinates)
    }
}

impl Mul<Hex> for Hex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut coordinates = [0; 3];

        coordinates[0] = self.coordinates[0] * rhs.coordinates[0];
        coordinates[1] = self.coordinates[1] * rhs.coordinates[1];
        coordinates[2] = self.coordinates[2] * rhs.coordinates[2];

        Self::from(coordinates)
    }
}

pub struct FractionalHex {
    coordinates: [f64; 3],
}

impl FractionalHex {
    pub fn new(coordinates: [f64; 3]) -> Self {
        Self { coordinates }
    }

    pub fn q(&self) -> f64 {
        self.coordinates[0]
    }

    pub fn r(&self) -> f64 {
        self.coordinates[1]
    }

    pub fn s(&self) -> f64 {
        self.coordinates[2]
    }
}
