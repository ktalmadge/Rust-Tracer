extern crate cgmath;

use self::cgmath::*;

use object::Object;

#[derive(Clone, Copy)]
pub struct Triangle {
    p1: Vector3<f64>,
    p2: Vector3<f64>,
    p3: Vector3<f64>,
}

impl Triangle {
    pub fn new(p1: Vector3<f64>, p2: Vector3<f64>, p3: Vector3<f64>) -> Triangle {
        Triangle { p1, p2, p3 }
    }
}

impl Object for Triangle {
    fn intersect(&self, ray: ::ray::Ray) -> bool {
        let eps: f64 = 0.0001;

        let v1v2: Vector3<f64> = self.p2 - self.p1;
        let v1v3: Vector3<f64> = self.p3 - self.p1;

        let h: Vector3<f64> = ray.direction.cross(v1v3);
        let a: f64 = v1v2.dot(h);

        if a.abs() < eps {
            return false;
        }

        let f: f64 = 1f64 / a;
        let s: Vector3<f64> = ray.origin - self.p1;

        let u: f64 = f * s.dot(h);

        if u < 0f64 || u > 1f64 {
            return false;
        }

        let q: Vector3<f64> = s.cross(v1v2);
        let v: f64 = f * ray.direction.dot(q);

        if v < 0f64 || u + v > 1f64 {
            return false;
        }

        let t: f64 = f * v1v3.dot(q);

        if t > eps {
            return true;
        }

        false
    }
}
