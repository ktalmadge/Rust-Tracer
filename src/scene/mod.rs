extern crate image;
extern crate cgmath;

use self::cgmath::*;

use self::image::{Pixel, Rgba};

use std::f64;

mod view_window;

use light::Light;
use camera::Camera;
use object::Object;
use pixel_buffer::PixelBuffer;
use ray::Ray;
use self::view_window::ViewWindow;

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
    pub fn new(
        width: usize,
        height: usize,
        lights: Vec<Box<Light>>,
        objects: Vec<Box<Object>>,
        ambient_coefficient: f64,
    ) -> Scene {
        let aspect_ratio: f64 = width as f64 / height as f64;

        Scene {
            scene_contents: SceneContents { lights, objects },
            scene_characteristics: SceneCharacteristics {
                ambient_coefficient,
                diffuse_coefficient: 1f64 - ambient_coefficient,
            },
            camera: Camera::new(
                Vector3::new(0f64, 0f64, -1f64),
                Vector3::new(0f64, 0f64, 0f64),
            ),
            pixel_buffer: PixelBuffer::new(width, height),
            view_window: ViewWindow::new(width, height, 2f64, Vector3::new(0f64, 0f64, 0f64)),
        }
    }

    // must find closest intersection
    fn closest_intersection(&self, ray: &Ray) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut shortest_distance: f64 = f64::MAX;

        for object in self.scene_contents.objects.iter() {
            if let Some(intersection) = object.intersect(ray) {
                let distance: f64 = (intersection - ray.origin).magnitude();
                if shortest_distance > distance {
                    shortest_distance = distance;
                    result = Some(RayHit {
                        object: &object,
                        intersection,
                        distance,
                    });
                }
            }
        }

        result
    }

    fn min_u8(&self, n1: u8, n2: u8) -> u8 {
        if n1 < n2 { n1 } else { n2 }
    }

    fn mul_color(&self, color: Rgba<u8>, value: f64) -> Rgba<u8> {
        Rgba(
            [
                (color.data[0] as f64 * value) as u8,
                (color.data[1] as f64 * value) as u8,
                (color.data[2] as f64 * value) as u8,
                color.data[3],
            ],
        )
    }

    fn add_color(&self, color: Rgba<u8>, other: Rgba<u8>) -> Rgba<u8> {
        Rgba(
            [
                self.min_u8(color.data[0] + other.data[0], 255),
                self.min_u8(color.data[1] + other.data[0], 255),
                self.min_u8(color.data[2] + other.data[0], 255),
                255u8,
            ],
        )
    }

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Rgba<u8> {
        let obj_color: Rgba<u8> = Rgba::from_channels(255, 255, 255, 255);

        let mut result: Rgba<u8> =
            self.mul_color(obj_color, self.scene_characteristics.ambient_coefficient);

        for light in self.scene_contents.lights.iter() {
            let from_light: Ray = Ray::new(light.origin, ray_hit.intersection);
            let shade: f64 = 1f64 - ray.direction.dot(from_light.direction);
            if shade > 0f64 {
                result =
                    self.add_color(
                        self.mul_color(
                            obj_color,
                            self.scene_characteristics.diffuse_coefficient * shade,
                        ),
                        self.mul_color(obj_color, self.scene_characteristics.ambient_coefficient),
                    );
            }
        }

        result
    }

    fn trace(&mut self, ray: &Ray) -> Option<Rgba<u8>> {
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
                    self.pixel_buffer.set_pixel_rgba(x, y, color);
                }
            }
        }

        self.pixel_buffer.save_image("img/scene.png").unwrap();
    }
}
