extern crate cgmath;

use self::cgmath::*;

use object::Object;
use std::f64;

#[derive(Clone, Copy, Debug)]
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
    fn normal(&self, intersection: Vector3<f64>, incoming_vector: Vector3<f64>) -> Vector3<f64> {
        (intersection - self.origin).normalize()
    }

    fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>> {
        let diff: Vector3<f64> = ray.origin - self.origin;
        let b: f64 = 2f64 * diff.dot(ray.direction);
        let c: f64 = diff.dot(diff) - self.radius.powi(2);
        let root = b.powi(2) - 4f64 * c;

        if root < 0f64 {
            None
        } else {
            // w: Distance along ray to intersection
            let w = ((-b + root.sqrt()) / 2f64).min((-b - root.sqrt()) / 2f64);

            Some(Vector3::new(
                ray.origin.x + ray.direction.x * w,
                ray.origin.y + ray.direction.y * w,
                ray.origin.z + ray.direction.z * w,
            ))
        }
    }
}
