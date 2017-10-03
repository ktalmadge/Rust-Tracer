pub mod sphere;
pub mod triangle;

pub trait Object {
    fn intersect(&self, ray: ::ray::Ray) -> bool;
}
