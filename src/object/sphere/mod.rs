extern crate cgmath;

use self::cgmath::*;

use object::Object;

pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64) -> Sphere {
        Sphere { origin, radius }
    }
}

impl Object for Sphere {
    fn intersect(&mut self, ray: ::ray::Ray) -> bool {
        true
    }
}
