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
mod ray_tracer;

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
    fn test_ray_tracer() {
        let mut sphere: Box<Object> = Box::new(object::sphere::Sphere::new(
            Vector3::new(0f64, 0f64, 3f64),
            1f64,
        ));

        let mut triangle: Box<Object> = Box::new(object::triangle::Triangle::new(
            Vector3::new(-1f64, -1f64, 3f64),
            Vector3::new(-1f64, 1f64, 3f64),
            Vector3::new(1f64, 1f64, 3f64),
        ));

        let mut objects: Vec<Box<Object>> = Vec::new();
        objects.push(triangle);
        objects.push(sphere);

        let mut ray_tracer = ray_tracer::RayTracer::new(WIDTH, HEIGHT, objects);

        ray_tracer.trace();
    }

    #[test]
    fn test_ray() {
        let ray: ray::Ray = ray::Ray::new(
            Vector3::new(0f64, 0f64, 0f64),
            Vector3::new(1f64, 1f64, 1f64),
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
            Vector3::new(2f64, 2f64, 2f64),
        );

        let ray_miss: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0f64, 0f64, 1f64),
        );

        assert!(sphere.intersect(ray_hit).is_some());
        assert!(sphere.intersect(ray_miss).is_none());
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
            Vector3::new(2f64, 2f64, 2f64),
        );

        let ray_miss: ray::Ray = ray::Ray::new(
            Vector3::new(1f64, 1f64, 1f64),
            Vector3::new(0f64, 0f64, 1f64),
        );

        assert!(triangle.intersect(ray_hit).is_some());
        assert!(triangle.intersect(ray_miss).is_none());
    }

    fn generate_triangle() -> Box<Object> {
        Box::new(object::triangle::Triangle::new(
            Vector3::new(4f64, 6f64, 4f64),
            Vector3::new(6f64, 4f64, 4f64),
            Vector3::new(5f64, 5f64, 6f64),
        ))
    }

    fn generate_sphere() -> Box<Object> {
        Box::new(object::sphere::Sphere::new(
            Vector3::new(4f64, 6f64, 4f64),
            1f64,
        ))
    }

    #[test]
    fn test_shapes() {
        let mut triangle: Box<Object> = generate_triangle();
        let mut sphere: Box<Object> = generate_sphere();
    }
}
