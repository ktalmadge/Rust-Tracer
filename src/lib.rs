#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;
extern crate image;

#[macro_use]
extern crate serde_derive;

mod camera;
mod color;
mod light;
mod object;
mod pixel_buffer;
mod ray;
mod reader;
mod scene;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

#[cfg(test)]
mod tests {
    use super::*;

    use super::cgmath::Vector3;

    #[test]
    fn test_ray() {
        let straight_ray: ray::Ray = ray::Ray::new(
            Vector3::new(0f64, 0f64, 0f64),
            Vector3::new(0f64, 0f64, -1f64),
        );
        let straight_normal: Vector3<f64> = Vector3::new(0f64, 0f64, 1f64);

        assert_eq!(straight_ray.reflection(straight_normal), straight_normal);
    }
}
