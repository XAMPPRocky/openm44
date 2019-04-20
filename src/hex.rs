use std::ops::{Add, Mul, Sub};

const HEX_DIRECTIONS: [Hex; 6] = [
    Hex::new_unchecked(1, 0, -1), Hex::new_unchecked(1, -1, 0), Hex::new_unchecked(0, -1, 1),
    Hex::new_unchecked(-1, 0, 1), Hex::new_unchecked(-1, 1, 0), Hex::new_unchecked(0, 1, -1),
];

#[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, PartialEq)]
pub struct Hex {
    coordinates: [i8; 3],
}

impl Hex {
    pub fn new(q: i8, r: i8, s: i8) -> Self {
        assert!(q + r + s == 0, "Invalid hex position: {} + {} + {} != 0", q, r, s);

        Hex { coordinates: [q, r, s] }
    }

    /// This exists solely because `assert_eq!` is not allowed in const
    /// expressions (rustc 1.33.0).
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

    fn len(self) -> usize {
        (self.coordinates.iter().fold(0, |acc, i| acc + i.abs()) / 2) as usize
    }

    pub fn distance(self, rhs: Self) -> usize {
        (self - rhs).len()
    }

    pub fn direction(direction: usize) -> Self {
        assert!(direction < 6);

        HEX_DIRECTIONS[direction]
    }

    pub fn neighbour(self, direction: usize) -> Self {
        self + Self::direction(direction)
    }

    pub fn lerp(self, rhs: Self, t: f32) -> FractionalHex {
        fn f32_lerp(a: f32, b: f32, t: f32) -> f32 {
            a * (1.0-t) + b * t
        }

        FractionalHex::new([ f32_lerp(self.q() as f32, rhs.q() as f32, t),
                             f32_lerp(self.r() as f32, rhs.r() as f32, t),
                             f32_lerp(self.s() as f32, rhs.s() as f32, t)])
    }

    pub fn linedraw(self, rhs: Self) -> Vec<Hex> {
        let distance = self.distance(rhs);
        let step = 1.0 / usize::max(distance, 1) as f32;

        (0..=distance).map(|i| self.lerp(rhs, step * i as f32).round()).collect()
    }
}

impl From<crate::offset::OffsetCoord> for Hex {
    fn from(offset: crate::offset::OffsetCoord) -> Self {
        let q = offset.col - (offset.row + crate::offset::OFFSET * (offset.row & 1) / 2);
        let r = offset.row;
        let s = -q - r;

        Self::new(q, r, s)
    }
}

impl Add<Hex> for Hex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let q = self.coordinates[0] + rhs.coordinates[0];
        let r = self.coordinates[1] + rhs.coordinates[1];
        let s = self.coordinates[2] + rhs.coordinates[2];

        Self::new(q, r, s)
    }
}

impl Sub<Hex> for Hex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let q = self.coordinates[0] - rhs.coordinates[0];
        let r = self.coordinates[1] - rhs.coordinates[1];
        let s = self.coordinates[2] - rhs.coordinates[2];

        Self::new(q, r, s)
    }
}

impl Mul<Hex> for Hex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let q = self.coordinates[0] * rhs.coordinates[0];
        let r = self.coordinates[1] * rhs.coordinates[1];
        let s = self.coordinates[2] * rhs.coordinates[2];

        Self::new(q, r, s)
    }
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub struct FractionalHex {
    coordinates: [f32; 3],
}

impl FractionalHex {
    pub fn new(coordinates: [f32; 3]) -> Self {
        Self { coordinates }
    }

    pub fn q(&self) -> f32 {
        self.coordinates[0]
    }

    pub fn r(&self) -> f32 {
        self.coordinates[1]
    }

    pub fn s(&self) -> f32 {
        self.coordinates[2]
    }

    pub fn round(self) -> Hex {
        let q = self.q().round();
        let r = self.r().round();
        let s = self.s().round();

        let q_diff = f32::abs(q - self.q());
        let r_diff = f32::abs(r - self.r());
        let s_diff = f32::abs(s - self.s());

        let mut q = q as i8;
        let mut r = r as i8;
        let mut s = s as i8;

        if q_diff > r_diff && q_diff > s_diff {
            q = -r - s;
        } else if r_diff > s_diff {
            r = -q - s;
        } else {
            s = -q - r;
        }

        Hex::new(q, r, s)
    }
}
