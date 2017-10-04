extern crate cgmath;
extern crate image;

use self::cgmath::*;
use self::image::Rgba;

pub struct Light {
    pub origin: Vector3<f64>,
    pub luminosity: f64,
    pub color: Rgba<u8>,
}

impl Light {
    pub fn new(origin: Vector3<f64>, luminosity: f64, color: Rgba<u8>) -> Light {
        Light {
            origin,
            luminosity,
            color,
        }
    }
}
