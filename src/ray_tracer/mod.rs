extern crate image;
extern crate cgmath;

use self::cgmath::*;

use self::image::{Pixel, Rgba};

use std::f64;

mod view_window;

use camera::Camera;
use object::Object;
use pixel_buffer::PixelBuffer;
use ray::Ray;
use self::view_window::ViewWindow;

pub struct RayTracer {
    width: usize,
    height: usize,
    objects: Vec<Box<Object>>,
    pixel_buffer: PixelBuffer,
    camera: Camera,
    view_window: ViewWindow,
}

struct RayHit<'a> {
    object: &'a Box<Object>,
    intersection: Vector3<f64>,
    distance: f64,
}

impl RayTracer {
    pub fn new(width: usize, height: usize, objects: Vec<Box<Object>>) -> RayTracer {
        let aspect_ratio: f64 = width as f64 / height as f64;

        RayTracer {
            width,
            height,
            objects,
            pixel_buffer: PixelBuffer::new(width, height),
            camera: Camera::new(
                Vector3::new(0f64, 0f64, 0f64),
                Vector3::new(0f64, 0f64, 1f64),
            ),
            view_window: ViewWindow::new(width, height, 2f64, Vector3::new(0f64, 0f64, 1f64)),
        }
    }

    // must find closest intersection
    fn closest_intersection(&mut self, ray: &Ray) -> Option<RayHit> {
        let mut result: Option<RayHit> = None;
        let mut shortest_distance: f64 = f64::MAX;

        for object in self.objects.iter() {
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

    fn trace(&mut self, ray: &Ray) -> Option<Rgba<u8>> {
        match self.closest_intersection(ray) {
            // Todo: something interesting when we get a ray hit
            Some(ray_hit) => Some(ray_hit.object.color()),
            None => None,
        }
    }

    pub fn draw(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray: Ray = Ray::new(self.camera.origin, self.view_window.at(x, y));

                if let Some(color) = self.trace(&ray) {
                    self.pixel_buffer.set_pixel_rgba(x, y, color);
                }
            }
        }

        self.pixel_buffer
            .save_image("img/ray_tracing_result.png")
            .unwrap();
    }
}
