extern crate cgmath;

use self::cgmath::*;

use object::material::Material;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Triangle {
    p1: Vector3<f64>,
    p2: Vector3<f64>,
    p3: Vector3<f64>,
    pub material: Material,
    pub normal: Vector3<f64>,
}

impl Triangle {
    pub fn new(
        p1: Vector3<f64>,
        p2: Vector3<f64>,
        p3: Vector3<f64>,
        material: Material,
    ) -> Triangle {
        Triangle {
            p1,
            p2,
            p3,
            material,
            normal: (p2 - p1).cross(p3 - p1).normalize(),
        }
    }

    pub fn normal(
        &self,
        intersection: Vector3<f64>,
        incoming_vector: Vector3<f64>,
    ) -> Vector3<f64> {
        if incoming_vector.dot(self.normal) > 0f64 {
            -self.normal
        } else {
            self.normal
        }
    }

    pub fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>> {
        let eps: f64 = 0.000_000_000_01;

        let v1v2: Vector3<f64> = self.p2 - self.p1;
        let v1v3: Vector3<f64> = self.p3 - self.p1;

        let h: Vector3<f64> = ray.direction.cross(v1v3);
        let a: f64 = v1v2.dot(h);

        if a.abs() < eps {
            return None;
        }

        let f: f64 = 1f64 / a;
        let s: Vector3<f64> = ray.origin - self.p1;

        let u: f64 = f * s.dot(h);

        if u < 0f64 || u > 1f64 {
            return None;
        }

        let q: Vector3<f64> = s.cross(v1v2);
        let v: f64 = f * ray.direction.dot(q);

        if v < 0f64 || u + v > 1f64 {
            return None;
        }

        // t: Distance along ray to intersection
        let t: f64 = f * v1v3.dot(q);

        if t > eps {
            return Some(Vector3::new(
                ray.origin.x + ray.direction.x * t,
                ray.origin.y + ray.direction.y * t,
                ray.origin.z + ray.direction.z * t,
            ));
        }

        None
    }
}
