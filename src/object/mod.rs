extern crate cgmath;

use self::cgmath::*;

pub mod material;
pub mod sphere;
pub mod triangle;

#[derive(Copy, Clone, PartialEq)]
pub enum Shape {
    Triangle(triangle::Triangle),
    Sphere(sphere::Sphere),
}

impl Shape {
    pub fn normal(
        &self,
        intersection: Vector3<f64>,
        incoming_vector: Vector3<f64>,
    ) -> Vector3<f64> {
        match *self {
            Shape::Triangle(triangle) => triangle.normal(intersection, incoming_vector),
            Shape::Sphere(sphere) => sphere.normal(intersection, incoming_vector),
        }
    }
    pub fn intersect(&self, ray: &::ray::Ray) -> Option<Vector3<f64>> {
        match *self {
            Shape::Triangle(triangle) => triangle.intersect(ray),
            Shape::Sphere(sphere) => sphere.intersect(ray),
        }
    }
    pub fn material(&self) -> material::Material {
        match *self {
            Shape::Triangle(triangle) => triangle.material,
            Shape::Sphere(sphere) => sphere.material,
        }
    }
}
