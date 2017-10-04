extern crate cgmath;

use self::cgmath::*;

pub mod sphere;
pub mod triangle;

pub trait Object {
    fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>>;
}
