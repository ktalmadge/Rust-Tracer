#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;

#[allow(unused_imports)]
use self::cgmath::Vector3;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

mod camera;
mod object;
mod pixel_buffer;
mod ray;

#[allow(unused_imports)]
use object::Object;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pixel_buffer() {
        let mut buffer: pixel_buffer::PixelBuffer = pixel_buffer::PixelBuffer::new(WIDTH, HEIGHT);

        buffer.set_pixel(200, 100, 255, 0, 0, 255);

        buffer.save_image("img/buff_out_vec.png").unwrap();
    }

    #[test]
    fn test_camera() {
        let camera = camera::Camera::new(
            Vector3::new(0f64, 0f64, 0f64),
            Vector3::new(0f64, 1f64, 0f64),
        );
    }

    #[test]
    fn test_ray() {
        let ray: ray::Ray = ray::Ray::new(
            Vector3::new(0f64, 0f64, 0f64),
            Vector3::new(0f64, 1f64, 0f64),
        );
    }

    #[test]
    fn test_objects() {
        let sphere: object::sphere::Sphere =
            object::sphere::Sphere::new(Vector3::new(0f64, 0f64, 0f64), 1f64);

        let triangle: object::triangle::Triangle = object::triangle::Triangle::new(
            Vector3::new(1f64, 0f64, 0f64),
            Vector3::new(0f64, 2f64, 0f64),
            Vector3::new(0f64, 0f64, 1f64),
        );
    }

    #[test]
    fn test_sphere_intersection() {
        let mut sphere: object::sphere::Sphere =
            object::sphere::Sphere::new(Vector3::new(5f64, 5f64, 5f64), 1f64);

        let ray_hit: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 1f64, 1f64),
        );

        let ray_miss: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0f64, 0f64, 1f64),
        );

        assert_eq!(sphere.intersect(ray_hit), true);
        assert_eq!(sphere.intersect(ray_miss), false);
    }

    #[test]
    fn test_triangle_intersection() {
        let mut triangle: object::triangle::Triangle = object::triangle::Triangle::new(
            Vector3::new(4f64, 6f64, 4f64),
            Vector3::new(6f64, 4f64, 4f64),
            Vector3::new(5f64, 5f64, 6f64),
        );

        let ray_hit: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(1f64, 1f64, 1f64),
        );

        let ray_miss: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0f64, 0f64, 1f64),
        );

        assert_eq!(triangle.intersect(ray_hit), true);
        assert_eq!(triangle.intersect(ray_miss), false);
    }
}
