extern crate cgmath;

use self::cgmath::*;

#[derive(Clone, Copy)]
pub struct Ray {
    pub origin: Vector3<f64>,
    pub direction: Vector3<f64>,
    pub inv_dir: Vector3<f64>,
}

impl Ray {
    // Generate a normalized ray from origin to destination
    pub fn new(origin: Vector3<f64>, direction: Vector3<f64>) -> Ray {
        Ray {
            origin,
            direction,
            inv_dir: 1f64 / direction,
        }
    }

    // Generate a normalized ray from origin to destination
    pub fn from_points(origin: Vector3<f64>, destination: Vector3<f64>) -> Ray {
        let direction: Vector3<f64> = (destination - origin).normalize();
        Ray {
            origin,
            direction,
            inv_dir: 1f64 / direction,
        }
    }

    pub fn distance(&self, other: Vector3<f64>) -> f64 {
        f64::from((other - self.origin).magnitude())
    }

    pub fn reflect(direction: Vector3<f64>, normal: Vector3<f64>) -> Vector3<f64> {
        direction - 2f64 * normal * direction.dot(normal)
    }

    pub fn reflection(&self, normal: Vector3<f64>) -> Vector3<f64> {
        Ray::reflect(self.direction, normal)
    }

    pub fn reflection_ray(&self, intersection: Vector3<f64>, normal: Vector3<f64>) -> Ray {
        let direction: Vector3<f64> = Ray::reflect(self.direction, normal);
        Ray {
            origin: intersection,
            direction,
            inv_dir: 1f64 / direction,
        }
    }
}
