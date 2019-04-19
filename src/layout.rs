use ggez::{GameResult, Context};
use ggez::graphics::{self, Point2 as Point, Color, DrawMode, Text};

use crate::{hex2::{FractionalHex, Hex}, hsl::Hsl, orientation::Orientation};

const PLAIN: Hsl = Hsl { hue: 79., saturation: 0.45, lightness: 0.5 };

pub struct Layout {
    pub orientation: &'static Orientation,
    pub size: Point,
    pub origin: Point,
}

impl Layout {
    pub fn new(orientation: &'static Orientation, size: Point, origin: Point) -> Self {
        Self { orientation, size, origin }
    }

    pub fn hex_to_pixel(&self, hex: Hex) -> Point {
        let matrix = self.orientation.matrix;
        let (q, r) = (hex.q() as f32, hex.r() as f32);

        let x = (matrix[0] * q + matrix[1] * r) * (self.size.x as f32);
        let y = (matrix[2] * q + matrix[3] * r) * (self.size.y as f32);

        Point::new(x + self.origin.x, y + self.origin.y)
    }

    pub fn pixel_to_hex(&self, point: Point) -> FractionalHex {
        let matrix = self.orientation.inverse;
        let point = Point::new(
            (point.x - self.origin.x) / self.size.x,
            (point.y - self.origin.y) / self.size.y,
        );

        let q = matrix[0] * point.x + matrix[1] * point.y;
        let r = matrix[2] * point.x + matrix[3] * point.y;

        FractionalHex::new([q, r, -q - r])
    }

    fn hex_corner_offset(&self, corner: i8) -> Point {
        let angle = 2.0 * std::f32::consts::PI *
            (self.orientation.start_angle + corner as f32) / 6.0;

        Point::new(self.size.x * angle.cos(), self.size.y * angle.sin())
    }

    pub fn polygon_corners(&self, hex: Hex) -> [Point; 6] {
        let mut points = [Point::new(0.0, 0.0); 6];
        let center = self.hex_to_pixel(hex);

        for (i, point) in points.iter_mut().enumerate() {
            let offset = self.hex_corner_offset(i as i8);

            point.x = center.x + offset.x;
            point.y = center.y + offset.y;
        }

        points
    }

    pub fn draw_hex(&mut self, ctx: &mut Context, hex: Hex) -> GameResult<()> {
        let corners = self.polygon_corners(hex);

        graphics::set_color(ctx, PLAIN.into())?;
        graphics::polygon(ctx, DrawMode::Fill, &corners)?;
        graphics::set_color(ctx, Color::new(0., 0., 0., 1.))?;
        graphics::polygon(ctx, DrawMode::Line(1.), &corners)?;
        Ok(())
    }
}

impl Default for Layout {
    fn default() -> Self {
        Self {
            orientation: &*crate::orientation::POINTY,
            size: Point::new(50.0, 50.0),
            origin: Point::new(50.0, 50.0)
        }
    }
}
