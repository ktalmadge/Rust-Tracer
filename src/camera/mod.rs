extern crate cgmath;

use self::cgmath::*;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Camera {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Camera {
        Camera { origin, direction }
    }
}
