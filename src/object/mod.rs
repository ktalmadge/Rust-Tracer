pub mod sphere;
pub mod triangle;

pub trait Object {
    fn intersect(&mut self, ray: ::ray::Ray) -> bool;
}
