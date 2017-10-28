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

    pub fn to_rgba(&self) -> Rgba<u8> {
        Rgba::from_channels(
            (self.r * 255f64) as u8,
            (self.g * 255f64) as u8,
            (self.b * 255f64) as u8,
            255,
        )
    }

    pub fn to_luminance(&self) -> f64 {
        // Magic numbers from  https://en.wikipedia.org/wiki/Relative_luminance
        self.r * 0.2126 + self.g * 0.7152 + self.b * 0.0722
    }
}

// Operator overloads

impl ops::Add<f64> for Color {
    type Output = Color;

    fn add(self, factor: f64) -> Color {
        Color::new(self.r + factor, self.g + factor, self.b + factor)
    }
}

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

impl ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color::new(self.r * other.r, self.g * other.g, self.b * other.b)
    }
}

impl ops::Div<f64> for Color {
    type Output = Color;

    fn div(self, factor: f64) -> Color {
        Color::new(self.r / factor, self.g / factor, self.b / factor)
    }
}

impl ops::Div<Color> for Color {
    type Output = Color;

    fn div(self, other: Color) -> Color {
        Color::new(self.r / other.r, self.g / other.g, self.b / other.b)
    }
}
