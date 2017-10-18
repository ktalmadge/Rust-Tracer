extern crate image;

use self::image::{Pixel, Rgba};

use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

fn cap(n: f64) -> f64 {
    if n > 255f64 { 255f64 } else { n }
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            r: cap(r),
            g: cap(g),
            b: cap(b),
        }
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(cap(self.r) as u8, cap(self.g) as u8, cap(self.b) as u8, 255)
    }
}

// Operator overloads

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(
            cap(self.r + other.r),
            cap(self.g + other.g),
            cap(self.b + other.b),
        )
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r = cap(self.r + other.r);
        self.g = cap(self.g + other.g);
        self.b = cap(self.b + other.b);
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, factor: f64) -> Color {
        Color::new(
            cap(cap(self.r) * factor),
            cap(cap(self.g) * factor),
            cap(cap(self.b) * factor),
        )
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, factor: f64) {
        self.r = cap(cap(self.r) * factor);
        self.g = cap(cap(self.g) * factor);
        self.b = cap(cap(self.b) * factor);
    }
}
