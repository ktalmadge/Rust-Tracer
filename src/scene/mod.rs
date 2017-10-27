#![cfg_attr(feature = "cargo-clippy", allow(borrowed_box))]

extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

pub mod configuration;
mod draw_iterator;
mod view_window;

use self::configuration::Configuration;
use self::draw_iterator::DrawIterator;
use self::view_window::ViewWindow;

use camera::Camera;
use color::Color;
use light::Light;
use object::*;
use ray::Ray;

pub struct Scene {
    camera: Camera,
    scene_contents: SceneContents,
    scene_characteristics: SceneCharacteristics,
    color_buffer: Vec<Vec<Color>>,
    view_window: ViewWindow,
}

struct SceneContents {
    lights: Vec<Light>,
    shapes: Vec<Shape>,
}

struct SceneCharacteristics {
    ambient_coefficient: f64,
    diffuse_coefficient: f64,
    specular_coefficient: f64,
    specular_exponent: f64,
    max_reflections: u8,
    reinhard_key_value: f64,
    reinhard_delta: f64,
}

struct RayHit<'a> {
    ray_direction: Vector3<f64>,
    shape: &'a Shape,
    intersection: Vector3<f64>,
    normal: Vector3<f64>,
    distance: f64,
}

impl Scene {
    pub fn new(configuration: &Configuration) -> Scene {
        /* Set up lights */
        let mut lights: Vec<Light> = Vec::new();
        for light_definition in &configuration.lights {
            lights.push(light_definition.as_light());
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
                diffuse_coefficient: configuration.diffuse_coefficient,
                specular_coefficient: configuration.specular_coefficient,
                specular_exponent: configuration.specular_exponent,
                max_reflections: configuration.max_reflections,
                reinhard_key_value: configuration.reinhard_key_value,
                reinhard_delta: configuration.reinhard_delta,
            },
            camera,
            color_buffer: vec![
                vec![Color::new(0f64, 0f64, 0f64); configuration.height];
                configuration.width
            ],
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
                        ray_direction: ray.direction,
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

    fn phong(&self, ray_hit: &RayHit, light: &Light, to_light: &Ray) -> Color {
        // TODO - Blend specular with material color

        // Diffuse component + specular component
        let reflection: Vector3<f64> = Ray::reflect(ray_hit.ray_direction, ray_hit.normal);
        let specular_component: Color = light.color * light.intensity *
            self.scene_characteristics.specular_coefficient *
            f64::max(0f64, to_light.direction.dot(reflection)).powf(
                self.scene_characteristics.specular_exponent,
            );

        let diffuse_component: Color = ray_hit.shape.material().color * light.intensity *
            self.scene_characteristics.diffuse_coefficient *
            f64::max(0f64, ray_hit.normal.dot(to_light.direction));

        diffuse_component + specular_component
    }

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let shape_color: Color = ray_hit.shape.material().color;

        let mut result: Color = shape_color * self.scene_characteristics.ambient_coefficient;

        for light in &self.scene_contents.lights {
            let to_light: Ray = Ray::from_points(ray_hit.intersection, light.origin);

            if self.shadow(ray_hit, &to_light) {
                continue;
            }

            result += self.phong(ray_hit, &light, &to_light);
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

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.color_buffer[x][y]
    }

    pub fn draw_iterator(&self, threads: usize, thread_number: usize) -> DrawIterator {
        DrawIterator::new(
            self.view_window.pixel_width,
            self.view_window.pixel_height,
            threads,
            thread_number,
        )
    }

    pub fn partial_draw(&mut self, threads: usize, thread_number: usize) {
        let iterator: DrawIterator = self.draw_iterator(threads, thread_number);

        for (x, y) in iterator {
            let mut ray: Ray = Ray::from_points(self.camera.origin, self.view_window.at(x, y));

            if let Some(color) = self.trace(&ray, 0u8) {
                self.color_buffer[x][y] = color;
            }
        }
    }

    pub fn draw(&mut self) {
        // Ray tracing for each pixel
        for x in 0..self.view_window.pixel_width {
            for y in 0..self.view_window.pixel_height {
                let mut ray: Ray = Ray::from_points(self.camera.origin, self.view_window.at(x, y));

                if let Some(color) = self.trace(&ray, 0u8) {
                    self.color_buffer[x][y] = color;
                }
            }
        }
    }
}
