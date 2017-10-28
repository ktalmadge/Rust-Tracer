extern crate cgmath;
extern crate image;

use self::cgmath::*;

use super::color::Color;

pub struct Light {
    pub origin: Vector3<f64>,
    pub intensity: f64,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Vector3<f64>, intensity: f64, color: Color) -> Light {
        Light {
            origin,
            intensity,
            color,
        }
    }
}
