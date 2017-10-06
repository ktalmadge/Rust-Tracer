extern crate image;
extern crate cgmath;

use self::cgmath::*;

use std::f64;

mod view_window;

use light::Light;
use camera::Camera;
use color::Color;
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

    fn light(&self, ray: &Ray, ray_hit: &RayHit) -> Color {
        let obj_color: Color = Color::new(200f64, 100f64, 100f64);

        let mut result: Color = obj_color * self.scene_characteristics.ambient_coefficient;

        for light in self.scene_contents.lights.iter() {
            let from_light: Ray = Ray::new(light.origin, ray_hit.intersection);
            let shade: f64 = 1f64 - ray.direction.dot(from_light.direction);
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
