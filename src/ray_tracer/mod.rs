extern crate image;
extern crate cgmath;

use self::cgmath::*;

use self::image::{Pixel, Rgba};

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

    // must find closest intersection


    pub fn trace(&mut self, ray: Ray) -> Option<Rgba<u8>> {
        for obj in self.objects.iter() {
            match obj.intersect(ray) {
                Some(intersection) => return Some(Rgba::from_channels(255, 0, 0, 255)),
                _ => (),
            }
        }

        None
    }

    pub fn draw(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                let mut ray: Ray = Ray::new(self.camera.origin, self.view_window.at(x, y));

                match self.trace(ray) {
                    Some(color) => self.pixel_buffer.set_pixel_rgba(x, y, color),
                    _ => {}
                }
            }
        }

        self.pixel_buffer
            .save_image("img/ray_tracing_result.png")
            .unwrap();
    }
}
