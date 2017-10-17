#![cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]

extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

mod view_window;
mod configuration;

use self::configuration::Configuration;
use light::Light;
use camera::Camera;
use color::Color;
use object::Object;
use pixel_buffer::PixelBuffer;
use ray::Ray;
use self::view_window::ViewWindow;
use object::sphere::Sphere;

pub struct Scene {
    camera: Camera,
    scene_contents: SceneContents,
    scene_characteristics: SceneCharacteristics,
    pixel_buffer: PixelBuffer,
    view_window: ViewWindow,
}

struct SceneContents {
    lights: Vec<Box<Light>>,
    objects: Vec<Box<Object>>,
}

struct SceneCharacteristics {
    ambient_coefficient: f64,
    diffuse_coefficient: f64,
}

struct RayHit<'a> {
    object: &'a Box<Object>,
    intersection: Vector3<f64>,
    distance: f64,
}

impl Scene {
    pub fn new(configuration_filename: String) -> Scene {
        let mut configuration: Configuration =
            Configuration::read_configuration(configuration_filename);

        /* Set up lights */
        let mut lights: Vec<Box<Light>> = Vec::new();
        for light_definition in &configuration.lights {
            lights.push(Box::new(light_definition.as_light()));
        }

        /*  Set up objects */
        let mut objects: Vec<Box<Object>> = Vec::new();
        for object_definition in &configuration.objects {
            objects.append(&mut (object_definition.read_objects()));
        }

        let mut sphere = Box::new(Sphere::new(Vector3::new(1.25f64, 0f64, 0f64), 0.5f64));
        objects.push(sphere);

        let camera: Camera = configuration.camera();
        let view_window_position: Vector3<f64> = camera.origin +
            (camera.target - camera.origin).normalize() * configuration.viewport_distance;

        Scene {
            scene_contents: SceneContents { lights, objects },
            scene_characteristics: SceneCharacteristics {
                ambient_coefficient: configuration.ambient_coefficient,
                diffuse_coefficient: 1f64 - configuration.ambient_coefficient,
            },
            camera,
            pixel_buffer: PixelBuffer::new(configuration.width, configuration.height),
            view_window: ViewWindow::new(
                configuration.width,
                configuration.height,
                configuration.viewport_width,
                view_window_position,
            ),
        }
    }

    // must find closest intersection
    fn closest_intersection(&self, ray: &Ray) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut shortest_distance: f64 = f64::MAX;

        for object in &self.scene_contents.objects {
            if let Some(intersection) = object.intersect(ray) {
                let distance: f64 = (intersection - ray.origin).magnitude();
                if shortest_distance > distance {
                    shortest_distance = distance;

                    // Offset a bit towards the camera to eliminate self-intersection
                    let intersection = intersection +
                        (self.camera.origin - intersection).normalize() * 0.000_01f64;
                    result = Some(RayHit {
                        object,
                        intersection,
                        distance,
                    });
                }
            }
        }

        result
    }

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let obj_color: Color = Color::new(200f64, 100f64, 100f64);

        let mut result: Color = obj_color * self.scene_characteristics.ambient_coefficient;

        for light in &self.scene_contents.lights {
            let to_light: Ray = Ray::new(ray_hit.intersection, light.origin);
            let mut normal: Vector3<f64> = ray_hit.object.normal(
                ray_hit.intersection,
                self.camera.orientation_vector(),
            );
            let shade: f64 = to_light.direction.dot(normal);
            if shade > 0f64 {
                result = obj_color * self.scene_characteristics.diffuse_coefficient * shade +
                    obj_color * self.scene_characteristics.ambient_coefficient;
            } else {
                result = obj_color * self.scene_characteristics.ambient_coefficient;
            }
        }

        result
    }

    fn trace(&mut self, ray: &Ray) -> Option<Color> {
        match self.closest_intersection(ray) {
            Some(ray_hit) => Some(self.light(ray, &ray_hit)),
            None => None,
        }
    }

    pub fn draw(&mut self) {
        for x in 0..self.view_window.pixel_width {
            for y in 0..self.view_window.pixel_height {
                let mut ray: Ray = Ray::new(self.camera.origin, self.view_window.at(x, y));

                if let Some(color) = self.trace(&ray) {
                    self.pixel_buffer.set_pixel(x, y, color);
                }
            }
        }

        self.pixel_buffer.save_image("img/scene.png").unwrap();
    }
}
