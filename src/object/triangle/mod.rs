extern crate cgmath;

use self::cgmath::*;

use object::Object;

pub struct Triangle {
    origin: Vector3<f64>,
    radius: f64,
}

impl Triangle {
    pub fn new(origin: Vector3<f64>, radius: f64) -> Triangle {
        Triangle { origin, radius }
    }
}

impl Object for Triangle {
    fn intersect(&mut self, ray: ::ray::Ray) -> bool {
        true
    }
}
