extern crate cgmath;

use self::cgmath::*;

pub struct Camera {
    pub origin: Vector3<f64>,
    pub target: Vector3<f64>,
    pub up: Vector3<f64>,
}

impl Camera {
    pub fn new(origin: Vector3<f64>, target: Vector3<f64>, up: Vector3<f64>) -> Camera {
        Camera { origin, target, up }
    }

    pub fn direction(&self) -> Vector3<f64> {
        (self.target - self.origin).normalize()
    }
}
