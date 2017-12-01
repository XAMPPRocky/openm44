use ggez::graphics::Color;

#[derive(Clone, Copy, Debug, Default)]
pub struct Hsl {
    /// Angle in degrees
    pub hue: f32,
    /// fraction between 0 - 1
    pub saturation: f32,
    /// fraction between 0 - 1
    pub lightness: f32,
}

impl Hsl {
    pub fn new(hue: f32, saturation: f32, lightness: f32) -> Self {
        assert!(hue >= 0. && saturation <= 360.);
        assert!(saturation >= 0. && saturation <= 1.);
        assert!(lightness >= 0. && lightness <= 1.);

        Hsl {
            hue,
            saturation,
            lightness,
        }
    }

    fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> u8 {
        if t < 0. {
            t += 1.;
        }

        if t > 1.  {
            t -= 1.;
        }

        let value = if t < 1./6. {
            p + (q - p) * 6. * t
        } else if t < 1./2. {
            q
        } else if t < 2./3. {
            p + (q - p) * (2./3. - t) * 6.
        } else {
            p
        };

        f32::round(value * 255.) as u8
    }

    pub fn to_rgb(self) -> Color {
        if self.saturation <= 0. {
            let value = self.lightness as u8;
            Color::from((value, value, value))
        } else {
            let hue = self.hue / 360.;
            let q = if self.lightness < 0.5 {
                self.lightness * (1. + self.saturation)
            } else {
                self.lightness + self.saturation -
                    self.lightness * self.saturation
            };

            let p = 2. * self.lightness - q;

            let r = Self::hue_to_rgb(p, q, hue + 1./3.);
            let g = Self::hue_to_rgb(p, q, hue);
            let b = Self::hue_to_rgb(p, q, hue - 1./3.);

            Color::from((r, g, b))
        }
    }

    pub fn darken(&mut self, by: f32) {
        self.lightness -= by;
    }

    pub fn brighten(&mut self, by: f32) {
        self.lightness += by;
    }

    pub fn is_tinted(&self) -> bool {
        self.lightness >= 0.50 && self.lightness <= 0.51
    }

    pub fn reset_tint(&mut self) {
        self.lightness = 0.5;
    }
}

impl From<Hsl> for Color {
    fn from(hsl: Hsl) -> Self {
        hsl.to_rgb()
    }
}
