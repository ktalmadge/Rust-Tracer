extern crate image;

use self::image::{Pixel, Rgba};

use std::ops;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Color {
    pub r: f64,
    pub g: f64,
    pub b: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color { r, g, b }
    }

    pub fn from_rgb(r: u8, g: u8, b: u8) -> Color {
        Color::new(
            f64::from(r) / 256f64,
            f64::from(g) / 256f64,
            f64::from(b) / 256f64,
        )
    }

    pub fn normalized(&self, total_light_intensity: f64) -> Color {
        Color::new(
            self.r / total_light_intensity * 255f64,
            self.g / total_light_intensity * 255f64,
            self.b / total_light_intensity * 255f64,
        )
    }

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(self.r as u8, self.g as u8, self.b as u8, 255)
    }
}

// Operator overloads

impl ops::Add<Color> for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color::new(self.r + other.r, self.g + other.g, self.b + other.b)
    }
}

impl ops::AddAssign<Color> for Color {
    fn add_assign(&mut self, other: Color) {
        self.r += other.r;
        self.g += other.g;
        self.b += other.b;
    }
}

impl ops::Mul<f64> for Color {
    type Output = Color;

    fn mul(self, factor: f64) -> Color {
        Color::new(self.r * factor, self.g * factor, self.b * factor)
    }
}

impl ops::MulAssign<f64> for Color {
    fn mul_assign(&mut self, factor: f64) {
        self.r *= factor;
        self.g *= factor;
        self.b *= factor;
    }
}
