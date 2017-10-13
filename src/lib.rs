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

    use object::Object;
    use super::cgmath::Vector3;
    use super::image::Rgba;

    #[test]
    fn draw_scene() {
        /*  Set up objects */
        let mut sphere1: Box<Object> = Box::new(object::sphere::Sphere::new(
            Vector3::new(2f64, 0f64, 0f64),
            1f64,
        ));

        let mut sphere2: Box<Object> = Box::new(object::sphere::Sphere::new(
            Vector3::new(0f64, 0f64, 3f64),
            1.0f64,
        ));

        let mut triangle1: Box<Object> = Box::new(object::triangle::Triangle::new(
            Vector3::new(-10f64, -10f64, 9f64),
            Vector3::new(-10f64, 10f64, 9f64),
            Vector3::new(10f64, 10f64, 9f64),
        ));

        let mut triangle2: Box<Object> = Box::new(object::triangle::Triangle::new(
            Vector3::new(-10f64, -10f64, 9f64),
            Vector3::new(10f64, -10f64, 9f64),
            Vector3::new(10f64, 10f64, 9f64),
        ));

        // Read in an object
        let mut r: reader::Reader = reader::Reader::new();
        assert!(r.read_file("./test/icosahedron.obj").is_ok());

        r.objects.push(sphere1);

        /* Set up lights */
        let mut light1: Box<light::Light> = Box::new(light::Light::new(
            Vector3::new(10f64, 10f64, 10f64),
            1f64,
            color::Color::new(255f64, 255f64, 255f64),
        ));

        let mut lights: Vec<Box<light::Light>> = Vec::new();
        lights.push(light1);

        /* Initiate and draw scene */
        let mut scene: scene::Scene = scene::Scene::new(WIDTH, HEIGHT, lights, r.objects, 0.4f64);
        scene.draw();
    }

    #[test]
    fn read_file() {
        let mut r: reader::Reader = reader::Reader::new();

        assert!(r.read_file("./test/cube.obj").is_ok());
        assert!(
            reader::Reader::new()
                .read_file("./test/missing_file.obj")
                .is_err()
        );
    }

    #[test]
    fn test_color() {
        // To RGBA
        let color_rgba: Rgba<u8> = color::Color::new(100f64, 150.5f64, 355.5f64).to_rgba();

        assert_eq!(color_rgba.data[0], 100);
        assert_eq!(color_rgba.data[1], 150);
        assert_eq!(color_rgba.data[2], 255);

        // Addition
        let color_sum: color::Color = color::Color::new(100f64, 50f64, 1000f64) +
            color::Color::new(100f64, 150.5f64, 355.5f64);

        assert_eq!(color_sum.r, 200f64);
        assert_eq!(color_sum.g, 200.5f64);
        assert_eq!(color_sum.b, 255f64);

        // Addition + assignment
        let mut color_sum: color::Color = color::Color::new(100f64, 50f64, 1000f64);
        color_sum += color::Color::new(100f64, 150.5f64, 355.5f64);

        assert_eq!(color_sum.r, 200f64);
        assert_eq!(color_sum.g, 200.5f64);
        assert_eq!(color_sum.b, 255f64);

        // Multiplication
        let half_color: color::Color = color::Color::new(100f64, 150.5f64, 355.5f64) * 0.5;

        assert_eq!(half_color.r, 50f64);
        assert_eq!(half_color.g, 75.25f64);
        assert_eq!(half_color.b, 127.5f64);

        // Multiplication + assignment
        let mut color: color::Color = color::Color::new(100f64, 150.5f64, 355.5f64);

        color *= 0.5;

        assert_eq!(color.r, 50f64);
        assert_eq!(color.g, 75.25f64);
        assert_eq!(color.b, 127.5f64);
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
