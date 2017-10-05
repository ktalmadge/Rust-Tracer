extern crate cgmath;
extern crate image;

use self::cgmath::*;

use color::Color;

pub struct Light {
    pub origin: Vector3<f64>,
    pub luminosity: f64,
    pub color: Color,
}

impl Light {
    pub fn new(origin: Vector3<f64>, luminosity: f64, color: Color) -> Light {
        Light {
            origin,
            luminosity,
            color,
        }
    }
}
