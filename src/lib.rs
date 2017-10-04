#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;
extern crate image;

#[allow(unused_imports)]
use self::cgmath::Vector3;

#[allow(unused_imports)]
use self::image::{Pixel, Rgba};

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

mod camera;
mod light;
mod object;
mod pixel_buffer;
mod ray;
mod scene;

#[allow(unused_imports)]
use object::Object;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_scene() {
        let mut sphere: Box<Object> = Box::new(object::sphere::Sphere::new(
            Vector3::new(0f64, 0f64, 3.6f64),
            2f64,
        ));

        let mut triangle: Box<Object> = Box::new(object::triangle::Triangle::new(
            Vector3::new(-2.5f64, -1f64, 3f64),
            Vector3::new(-2.5f64, 1f64, 3f64),
            Vector3::new(-1f64, 1f64, 3f64),
        ));

        let mut objects: Vec<Box<Object>> = Vec::new();
        objects.push(triangle);
        objects.push(sphere);

        let mut light1: Box<light::Light> = Box::new(light::Light::new(
            Vector3::new(-4f64, 0f64, 0f64),
            1f64,
            Rgba::from_channels(255, 255, 255, 255),
        ));

        let mut lights: Vec<Box<light::Light>> = Vec::new();
        lights.push(light1);

        let mut scene: scene::Scene = scene::Scene::new(WIDTH, HEIGHT, lights, objects, 0.05f64);

        scene.draw();
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

        assert!(sphere.intersect(&ray_hit).is_some());
        assert!(sphere.intersect(&ray_miss).is_none());
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

        assert!(triangle.intersect(&ray_hit).is_some());
        assert!(triangle.intersect(&ray_miss).is_none());
    }
}
