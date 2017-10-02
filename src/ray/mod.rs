extern crate cgmath;

use self::cgmath::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
}

impl Ray {
    // Generate a normalized ray from origin to destination
    pub fn new(origin: Vector3<f64>, destination: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction: (destination - origin).normalize(),
        }
    }
}
