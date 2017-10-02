pub mod sphere;
pub mod triangle;

pub trait Object {
    fn intersect(&mut self, ray: ::ray::Ray) -> bool;
}

pub enum Shape {
    Sphere(sphere::Sphere),
    Triangle(triangle::Triangle),
}
