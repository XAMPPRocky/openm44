use lazy_static::lazy_static;

lazy_static! {
    pub static ref POINTY: Orientation = Orientation::new(
        [f32::sqrt(3.0), f32::sqrt(3.0) / 2.0, 0.0, 3.0 / 2.0],
        [f32::sqrt(3.0) / 3.0, -1.0 / 3.0, 0.0, 2.0 / 3.0],
        0.5
    );
    pub static ref FLAT: Orientation = Orientation::new(
        [3.0 / 2.0, 0.0, f32::sqrt(3.0) / 2.0, f32::sqrt(3.0)],
        [2.0 / 3.0, 0.0, -1.0 / 3.0, f32::sqrt(3.0) / 3.0],
        0.0
    );
}

pub struct Orientation {
    pub matrix: [f32; 4],
    pub inverse: [f32; 4],
    pub start_angle: f32,
}

impl Orientation {
    const fn new(matrix: [f32; 4], inverse: [f32; 4], start_angle: f32) -> Self {
        Self { matrix, inverse, start_angle }
    }
}
