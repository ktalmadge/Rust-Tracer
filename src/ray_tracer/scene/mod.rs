extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

pub mod configuration;
mod draw_iterator;

use self::configuration::Configuration;
use self::draw_iterator::DrawIterator;

use super::camera::Camera;
use super::color::Color;
use super::light::Light;
use super::object::*;
use super::object::material::Material;
use super::ray::Ray;

pub struct Scene {
    camera: Camera,
    scene_contents: SceneContents,
    scene_characteristics: SceneCharacteristics,
    view_characteristics: ViewCharacteristics,
    color_buffer: Vec<Vec<Color>>,
}

struct SceneContents {
    lights: Vec<Light>,
    shapes: Vec<Shape>,
}

struct SceneCharacteristics {
    max_reflections: u8,
    reinhard_key_value: f64,
    reinhard_delta: f64,
}

struct ViewCharacteristics {
    pixel_width: usize,
    pixel_height: usize,
    viewport_width: f64,
    viewport_height: f64,
    viewport_distance: f64,
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

        /* Set up camera */
        let camera: Camera = configuration.camera();

        /* Calculate viewport height from aspect ratio */
        let viewport_height: f64 = (configuration.height as f64 / configuration.width as f64) *
            configuration.viewport_width;

        Scene {
            scene_contents: SceneContents { lights, shapes },
            scene_characteristics: SceneCharacteristics {
                max_reflections: configuration.max_reflections,
                reinhard_key_value: configuration.reinhard_key_value,
                reinhard_delta: configuration.reinhard_delta,
            },
            view_characteristics: ViewCharacteristics {
                pixel_width: configuration.width,
                pixel_height: configuration.height,
                viewport_width: configuration.viewport_width,
                viewport_height,
                viewport_distance: configuration.viewport_distance,
            },
            camera,
            color_buffer: vec![
                vec![Color::new(0f64, 0f64, 0f64); configuration.height];
                configuration.width
            ],
        }
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        self.color_buffer[x][y]
    }

    // Generate a ray from the camera through the viewport
    pub fn generate_ray(&self, x: usize, y: usize) -> Ray {
        let camera_position: Vector3<f64> = self.camera.origin;
        let camera_direction: Vector3<f64> = self.camera.direction();
        let camera_right: Vector3<f64> = camera_direction.cross(self.camera.up).normalize();
        let camera_up: Vector3<f64> = camera_direction.cross(camera_right).normalize();

        // normalize x and y from -0.5 to 0.5
        let normalized_x = (x as f64 / self.view_characteristics.pixel_width as f64) - 0.5;
        let normalized_y = (y as f64 / self.view_characteristics.pixel_height as f64) - 0.5;

        // camera position + x factor + y factor + viewport direction / distance
        let viewport_intersection: Vector3<f64> = self.camera.origin +
            normalized_x * camera_right * self.view_characteristics.viewport_width +
            normalized_y * camera_up * self.view_characteristics.viewport_height +
            camera_direction * self.view_characteristics.viewport_distance;

        Ray::from_points(self.camera.origin, viewport_intersection)
    }

    // Find the closest intersection (if any)
    fn intersection(&self, ray: &Ray, this_object: Option<Shape>) -> Option<RayHit> {
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
            }
        }

        result
    }

    // Check if there is anything between the object and the light
    fn shadow(&self, object: Shape, to_light: &Ray, light_distance: f64) -> bool {
        if let Some(shadow_hit) = self.intersection(to_light, Some(object)) {
            shadow_hit.distance < light_distance
        } else {
            false
        }
    }

    // Phong shading for determining diffuse + specular contribution
    fn phong(&self, ray_hit: &RayHit, light: &Light, to_light: &Ray) -> Color {
        let material: Material = ray_hit.shape.material();

        let reflection: Vector3<f64> = Ray::reflect(ray_hit.ray_direction, ray_hit.normal);
        let specular_component: Color = light.color * light.intensity *
            material.specular_coefficient *
            f64::max(0f64, to_light.direction.dot(reflection)).powf(material.specular_exponent);

        let diffuse_component: Color = light.color * light.intensity * material.color *
            material.diffuse_coefficient *
            f64::max(0f64, ray_hit.normal.dot(to_light.direction));

        diffuse_component + specular_component
    }

    // Use material characteristics and lighting to determine the color
    fn shade(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let material: Material = ray_hit.shape.material();
        let mut result: Color = Color::new(0f64, 0f64, 0f64);

        for light in &self.scene_contents.lights {
            let ambient_contribution: Color = light.color * light.intensity * material.color *
                material.ambient_coefficient;

            let to_light: Ray = Ray::from_points(ray_hit.intersection, light.origin);
            let light_distance: f64 = (light.origin - ray_hit.intersection).magnitude();

            if self.shadow(*ray_hit.shape, &to_light, light_distance) {
                result += ambient_contribution;
                continue;
            }

            result += ambient_contribution + self.phong(ray_hit, light, &to_light);
        }

        result
    }

    // Follow the ray to determine the color of the pixel
    fn trace(&self, ray: &Ray, reflection_level: u8) -> Option<Color> {
        match self.intersection(ray, None) {
            None => None,
            Some(ray_hit) => {
                let material: Material = ray_hit.shape.material();
                let mut object_color: Color = self.shade(ray, &ray_hit);
                if reflection_level < self.scene_characteristics.max_reflections &&
                    material.reflectance > 0f64
                {
                    // Object is reflective - recursively trace reflection ray
                    let reflection_ray =
                        Ray::new(ray_hit.intersection, ray.reflection(ray_hit.normal));

                    if let Some(reflection_color) =
                        self.trace(&reflection_ray, reflection_level + 1u8)
                    {
                        // Combine reflection color and object color
                        object_color = object_color * material.normal +
                            reflection_color * material.reflectance;
                    }
                }

                Some(object_color)
            }
        }
    }

    // Iterator for parallel draws to ensure each thread draws the correct
    // subset of the image and that the final combination step correctly selects
    // the image subset for each thread
    pub fn draw_iterator(&self, threads: usize, thread_number: usize) -> DrawIterator {
        DrawIterator::new(
            self.view_characteristics.pixel_width,
            self.view_characteristics.pixel_height,
            threads,
            thread_number,
        )
    }

    // Draw part of the image - used for multi-threaded tracing
    pub fn partial_draw(&mut self, threads: usize, thread_number: usize) {
        let iterator: DrawIterator = self.draw_iterator(threads, thread_number);

        for (x, y) in iterator {
            let mut ray: Ray = self.generate_ray(x, y);

            if let Some(color) = self.trace(&ray, 0u8) {
                self.color_buffer[x][y] = color;
            }
        }
    }

    // Draw the whole image
    pub fn draw(&mut self) {
        // Ray tracing for each pixel
        for x in 0..self.view_characteristics.pixel_width {
            for y in 0..self.view_characteristics.pixel_height {
                let mut ray: Ray = self.generate_ray(x, y);

                if let Some(color) = self.trace(&ray, 0u8) {
                    self.color_buffer[x][y] = color;
                }
            }
        }
    }
}
