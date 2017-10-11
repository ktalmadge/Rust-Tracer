extern crate cgmath;

use self::cgmath::*;

pub mod sphere;
pub mod triangle;

pub trait Object {
    fn normal(&self, intersection: Vector3<f64>) -> Vector3<f64>;
    fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>>;
}
