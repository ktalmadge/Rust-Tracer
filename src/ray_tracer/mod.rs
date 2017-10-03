extern crate cgmath;

use self::cgmath::*;

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

    pub fn generate_ray(&mut self, x: usize, y: usize) -> Ray {
        Ray::new(self.camera.origin, self.view_window.at(x, y))
    }

    pub fn trace(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray: Ray = self.generate_ray(x, y);

                for obj in self.objects.iter() {
                    match obj.intersect(ray) {
                        Some(intersection) => self.pixel_buffer.set_pixel(x, y, 255, 255, 255, 255),
                        _ => (),
                    }
                }
            }
        }

        self.pixel_buffer
            .save_image("img/ray_tracing_result.png")
            .unwrap();
    }
}
