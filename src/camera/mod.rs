extern crate cgmath;

use self::cgmath::*;

pub struct Camera {
    position: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Camera {
    pub fn new(position: Vector3<f64>, direction: Vector3<f64>) -> Camera {
        Camera {
            position,
            direction,
        }
    }
}
