#![cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]

extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

mod configuration;
mod pixel_buffer;
mod view_window;

use self::configuration::Configuration;
use self::pixel_buffer::PixelBuffer;
use self::view_window::ViewWindow;

use light::Light;
use camera::Camera;
use color::Color;
use object::*;
use ray::Ray;

pub struct Scene {
    camera: Camera,
    scene_contents: SceneContents,
    scene_characteristics: SceneCharacteristics,
    pixel_buffer: PixelBuffer,
    view_window: ViewWindow,
}

struct SceneContents {
    lights: Vec<Box<Light>>,
    shapes: Vec<Shape>,
}

struct SceneCharacteristics {
    ambient_coefficient: f64,
    diffuse_coefficient: f64,
    specular_exponent: f64,
    max_reflections: u8,
}

struct RayHit<'a> {
    shape: &'a Shape,
    intersection: Vector3<f64>,
    normal: Vector3<f64>,
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
        let mut shapes: Vec<Shape> = Vec::new();
        for object_definition in &configuration.objects {
            shapes.append(&mut (object_definition.read_shapes()));
        }

        /* Set up camera and view window */
        let camera: Camera = configuration.camera();
        let view_window_position: Vector3<f64> = camera.origin +
            (camera.target - camera.origin).normalize() * configuration.viewport_distance;

        Scene {
            scene_contents: SceneContents { lights, shapes },
            scene_characteristics: SceneCharacteristics {
                ambient_coefficient: configuration.ambient_coefficient,
                diffuse_coefficient: 1f64 - configuration.ambient_coefficient,
                specular_exponent: configuration.specular_exponent,
                max_reflections: configuration.max_reflections,
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
    fn intersection(
        &self,
        ray: &Ray,
        this_object: Option<Shape>,
        closest_intersection: bool,
    ) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut shortest_distance: f64 = f64::MAX;

        for shape in &self.scene_contents.shapes {
            if let Some(this_shape) = this_object {
                if *shape == this_shape {
                    continue;
                }
            }

            if let Some(intersection) = shape.intersect(ray) {
                let distance: f64 = (intersection - ray.origin).magnitude();
                if shortest_distance > distance {
                    shortest_distance = distance;

                    let normal: Vector3<f64> = shape.normal(intersection, ray.direction);

                    result = Some(RayHit {
                        shape,
                        intersection,
                        normal,
                        distance,
                    });
                }

                if !closest_intersection {
                    return result;
                }
            }
        }

        result
    }

    fn shadow(&self, ray_hit: &RayHit, to_light: &Ray) -> bool {
        if let Some(shadow_hit) = self.intersection(to_light, Some(*ray_hit.shape), false) {
            true
        } else {
            false
        }
    }

    fn phong(
        &self,
        incoming_ray: &Ray,
        to_light: &Ray,
        normal: Vector3<f64>,
        material: material::Material,
    ) -> Option<Color> {
        let shade: f64 = to_light.direction.dot(normal);

        if shade > 0f64 {
            // Specular component
            let mut result: Color = Color::new(100f64, 100f64, 100f64) *
                f64::max(
                    0f64,
                    to_light.direction.dot(incoming_ray.reflection(normal)),
                ).powf(self.scene_characteristics.specular_exponent);
            // Diffuse component
            result += material.color * self.scene_characteristics.diffuse_coefficient * shade;

            Some(result)
        } else {
            None
        }
    }

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let shape_color: Color = ray_hit.shape.material().color;

        let mut result: Color = shape_color * self.scene_characteristics.ambient_coefficient;

        for light in &self.scene_contents.lights {
            let to_light: Ray = Ray::from_points(ray_hit.intersection, light.origin);

            if self.shadow(ray_hit, &to_light) {
                continue;
            }

            if let Some(color) = self.phong(
                ray,
                &to_light,
                ray_hit.normal,
                ray_hit.shape.material(),
            )
            {
                result += color;
            }
        }

        result
    }

    fn trace(&self, ray: &Ray, reflection_level: u8) -> Option<Color> {
        match self.intersection(ray, None, true) {
            Some(ray_hit) => {
                let mut object_color: Color = self.light(ray, &ray_hit);
                if reflection_level < self.scene_characteristics.max_reflections &&
                    ray_hit.shape.material().reflective
                {
                    let reflection_ray =
                        Ray::new(ray_hit.intersection, ray.reflection(ray_hit.normal));

                    if let Some(reflection_color) =
                        self.trace(&reflection_ray, reflection_level + 1u8)
                    {
                        object_color += reflection_color;
                    }
                }

                Some(object_color)
            }
            None => None,
        }
    }

    pub fn draw(&mut self) {
        for x in 0..self.view_window.pixel_width {
            for y in 0..self.view_window.pixel_height {
                let mut ray: Ray = Ray::from_points(self.camera.origin, self.view_window.at(x, y));

                if let Some(color) = self.trace(&ray, 0u8) {
                    self.pixel_buffer.set_pixel(x, y, color);
                }
            }
        }

        self.pixel_buffer.save_image("img/scene.png").unwrap();
    }
}
