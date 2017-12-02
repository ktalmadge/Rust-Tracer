extern crate cgmath;
extern crate image;
extern crate rand;

use self::cgmath::*;

use std::f64;
use rand::Rng;

pub mod configuration;
mod draw_iterator;

use self::configuration::Configuration;
use self::draw_iterator::DrawIterator;

use super::camera::Camera;
use super::color::Color;
use super::intersection::Intersection;
use super::kd_tree::KdTree;
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
    kd_tree: KdTree,
}

struct SceneCharacteristics {
    samples: usize,
    max_reflections: u8,
    use_kd_tree: bool,
    reinhard_key_value: f64,
    reinhard_delta: f64,
}

struct ViewCharacteristics {
    pixel_width: usize,
    pixel_height: usize,
    viewport_width: f64,
    viewport_height: f64,
    width_tolerance: f64,
    half_width_tolerance: f64,
    height_tolerance: f64,
    half_height_tolerance: f64,
    viewport_distance: f64,
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

        let kd_tree: KdTree = KdTree::new(&shapes, configuration.max_kd_tree_depth);

        /* Set up camera */
        let camera: Camera = configuration.camera();

        /* Calculate viewport height from aspect ratio */
        let viewport_height: f64 = (configuration.height as f64 / configuration.width as f64) *
            configuration.viewport_width;

        /* Normalized pixel tolerance - for supersampling */
        let width_tolerance: f64 = 1f64 / configuration.width as f64;
        let height_tolerance: f64 = 1f64 / configuration.height as f64;

        Scene {
            scene_contents: SceneContents {
                lights,
                shapes,
                kd_tree,
            },
            scene_characteristics: SceneCharacteristics {
                samples: configuration.samples,
                max_reflections: configuration.max_reflections,
                use_kd_tree: configuration.use_kd_tree,
                reinhard_key_value: configuration.reinhard_key_value,
                reinhard_delta: configuration.reinhard_delta,
            },
            view_characteristics: ViewCharacteristics {
                pixel_width: configuration.width,
                pixel_height: configuration.height,
                viewport_width: configuration.viewport_width,
                viewport_height,
                width_tolerance,
                half_width_tolerance: width_tolerance / 2f64,
                height_tolerance,
                half_height_tolerance: height_tolerance / 2f64,
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
    pub fn generate_ray(&self, x: usize, y: usize, randomize: bool) -> Ray {
        let camera_position: Vector3<f64> = self.camera.origin;
        let camera_direction: Vector3<f64> = self.camera.direction();
        let camera_right: Vector3<f64> = camera_direction.cross(self.camera.up).normalize();
        let camera_up: Vector3<f64> = camera_direction.cross(camera_right).normalize();

        // normalize x and y from -0.5 to 0.5
        let mut normalized_x = (x as f64 / self.view_characteristics.pixel_width as f64) - 0.5;
        let mut normalized_y = (y as f64 / self.view_characteristics.pixel_height as f64) - 0.5;

        if randomize {
            let mut rng = rand::thread_rng();
            normalized_x += rng.gen::<f64>() * self.view_characteristics.width_tolerance -
                self.view_characteristics.half_width_tolerance;
            normalized_y += rng.gen::<f64>() * self.view_characteristics.height_tolerance -
                self.view_characteristics.half_height_tolerance;
        }

        // camera position + x factor + y factor + viewport direction / distance
        let viewport_intersection: Vector3<f64> = self.camera.origin +
            normalized_x * camera_right * self.view_characteristics.viewport_width +
            normalized_y * camera_up * self.view_characteristics.viewport_height +
            camera_direction * self.view_characteristics.viewport_distance;

        Ray::from_points(self.camera.origin, viewport_intersection)
    }

    // Find the closest intersection (if any)
    fn intersection(&self, ray: &Ray) -> Option<Intersection> {
        if self.scene_characteristics.use_kd_tree {
            return self.scene_contents.kd_tree.intersect(ray);
        }

        Intersection::closest_intersection(ray, &self.scene_contents.shapes)
    }

    // Check if there is anything between the object and the light
    fn shadow(&self, object: Shape, to_light: &Ray, light_distance: f64) -> bool {
        if let Some(shadow_intersection) = self.intersection(to_light) {
            shadow_intersection.distance < light_distance
        } else {
            false
        }
    }

    // Phong shading for determining diffuse + specular contribution
    fn phong(&self, ray_intersection: &Intersection, light: &Light, to_light: &Ray) -> Color {
        let material: Material = ray_intersection.shape.material();

        let reflection: Vector3<f64> =
            Ray::reflect(ray_intersection.ray_direction, ray_intersection.normal);
        let specular_component: Color = light.color * light.intensity *
            material.specular_coefficient *
            f64::max(0f64, to_light.direction.dot(reflection)).powf(material.specular_exponent);

        let diffuse_component: Color = light.color * light.intensity * material.color *
            material.diffuse_coefficient *
            f64::max(0f64, ray_intersection.normal.dot(to_light.direction));

        diffuse_component + specular_component
    }

    // Use material characteristics and lighting to determine the color
    fn shade(&self, ray: &Ray, ray_intersection: &Intersection) -> Color {
        let material: Material = ray_intersection.shape.material();
        let mut result: Color = Color::new(0f64, 0f64, 0f64);

        for light in &self.scene_contents.lights {
            let ambient_contribution: Color = light.color * light.intensity * material.color *
                material.ambient_coefficient;

            let to_light: Ray = Ray::from_points(ray_intersection.point, light.origin);
            let light_distance: f64 = (light.origin - ray_intersection.point).magnitude();

            if self.shadow(ray_intersection.shape, &to_light, light_distance) {
                result += ambient_contribution;
                continue;
            }

            result += ambient_contribution + self.phong(ray_intersection, light, &to_light);
        }

        result
    }

    // Follow the ray to determine the color of the pixel
    fn trace(&self, ray: &Ray, reflection_level: u8) -> Option<Color> {
        match self.intersection(ray) {
            None => None,
            Some(ray_intersection) => {
                let material: Material = ray_intersection.shape.material();
                let mut object_color: Color = self.shade(ray, &ray_intersection);
                if reflection_level < self.scene_characteristics.max_reflections &&
                    material.reflectance > 0f64
                {
                    // Object is reflective - recursively trace reflection ray
                    let reflection_ray = Ray::new(
                        ray_intersection.point,
                        ray.reflection(ray_intersection.normal),
                    );

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

    // Sample the given pixel by tracing one or more rays through it
    pub fn sample(&mut self, x: usize, y: usize) -> Color {
        let mut final_color: Color = Color::new(0f64, 0f64, 0f64);

        for s in 0..self.scene_characteristics.samples {
            let super_sample: bool = s > 0;
            let mut ray: Ray = self.generate_ray(x, y, super_sample);

            if let Some(color) = self.trace(&ray, 0u8) {
                final_color += color;
            }
        }

        final_color / self.scene_characteristics.samples as f64
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
            if x % 10 == 0 && y == 0 {
                println!("{}", x);
            }
            self.color_buffer[x][y] = self.sample(x, y);
        }
    }

    // Draw the whole image
    pub fn draw(&mut self) {
        // Ray tracing for each pixel
        for x in 0..self.view_characteristics.pixel_width {
            for y in 0..self.view_characteristics.pixel_height {
                self.color_buffer[x][y] = self.sample(x, y);
            }
        }
    }
}
