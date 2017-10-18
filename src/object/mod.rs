extern crate cgmath;

use self::cgmath::*;

use color::Color;

pub mod sphere;
pub mod triangle;

pub trait Object {
    fn normal(&self, intersection: Vector3<f64>, incoming_vector: Vector3<f64>) -> Vector3<f64>;
    fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>>;
    fn color(&self) -> Color;
}
