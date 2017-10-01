pub mod sphere;
pub mod triangle;

trait Object {
    fn intersect(&mut self, ray: ::ray::Ray) -> bool;
}
