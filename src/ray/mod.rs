extern crate cgmath;

use self::cgmath::*;

pub struct Ray {
    origin: Vector3<f64>,
    direction: Vector3<f64>,
}

impl Ray {
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray { origin, direction }
    }
}
