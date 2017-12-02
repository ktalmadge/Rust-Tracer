extern crate cgmath;

use self::cgmath::*;

use ray_tracer::ray::Ray;
use super::material::Material;
use std::f64;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere {
    origin: Vector3<f64>,
    radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(origin: Vector3<f64>, radius: f64, material: Material) -> Sphere {
        Sphere {
            origin,
            radius,
            material,
        }
    }

    pub fn normal(
        &self,
        intersection: Vector3<f64>,
        incoming_vector: Vector3<f64>,
    ) -> Vector3<f64> {
        (intersection - self.origin).normalize()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Vector3<f64>> {
        let diff = self.origin - ray.origin;

        let tca: f64 = diff.dot(ray.direction);

        if tca.is_sign_negative() {
            return None;
        }

        let d2: f64 = diff.dot(diff) - tca * tca;

        let radius_squared = self.radius * self.radius;

        if d2 > radius_squared {
            return None;
        }

        let thc: f64 = (radius_squared - d2).sqrt();

        let t: f64 = f64::min(tca + thc, tca - thc);

        if t.is_sign_negative() {
            return None;
        }

        Some(Vector3::new(
            ray.origin.x + ray.direction.x * t,
            ray.origin.y + ray.direction.y * t,
            ray.origin.z + ray.direction.z * t,
        ))
    }

    pub fn min_extent(&self) -> Vector3<f64> {
        Vector3::new(
            self.origin[0] - self.radius,
            self.origin[1] - self.radius,
            self.origin[2] - self.radius,
        )
    }

    pub fn max_extent(&self) -> Vector3<f64> {
        Vector3::new(
            self.origin[0] + self.radius,
            self.origin[1] + self.radius,
            self.origin[2] + self.radius,
        )
    }

    pub fn midpoint(&self) -> Vector3<f64> {
        self.origin
    }
}
