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

#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Triangle(triangle::Triangle),
    Sphere(sphere::Sphere),
}

impl Object for Shape {
    fn normal(&self, intersection: Vector3<f64>, incoming_vector: Vector3<f64>) -> Vector3<f64> {
        match *self {
            Shape::Triangle(triangle) => triangle.normal(intersection, incoming_vector),
            Shape::Sphere(sphere) => sphere.normal(intersection, incoming_vector),
        }
    }
    fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>> {
        match *self {
            Shape::Triangle(triangle) => triangle.intersect(ray),
            Shape::Sphere(sphere) => sphere.intersect(ray),
        }
    }
    fn color(&self) -> Color {
        match *self {
            Shape::Triangle(triangle) => triangle.color(),
            Shape::Sphere(sphere) => sphere.color(),
        }
    }
}
