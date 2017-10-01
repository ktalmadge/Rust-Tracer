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
        let diff: Vector3<f64> = ray.origin - self.origin;
        let b: f64 = 2f64 * diff.dot(ray.direction);
        let c: f64 = diff.dot(diff) - self.radius.powi(2);
        let root = b.powi(2) - 4f64 * c;

        root > 0f64
    }
}
