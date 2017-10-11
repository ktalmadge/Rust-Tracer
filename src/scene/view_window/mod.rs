extern crate cgmath;

use self::cgmath::*;

pub struct ViewWindow {
    pub pixel_width: usize,
    pub pixel_height: usize,
    view_width: f64,
    view_height: f64,
    origin: Vector3<f64>,
    start_x: f64,
    start_y: f64,
    pixel_size: f64,
}

impl ViewWindow {
    pub fn new(
        pixel_width: usize,
        pixel_height: usize,
        view_width: f64,
        origin: Vector3<f64>,
    ) -> ViewWindow {
        let view_height: f64 = view_width * (pixel_height as f64 / pixel_width as f64);
        ViewWindow {
            pixel_width,
            pixel_height,
            view_width,
            view_height,
            origin,
            start_x: origin.x - view_width / 2f64,
            start_y: origin.y + view_height / 2f64, // Start from top of y
            pixel_size: view_width / pixel_width as f64,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> Vector3<f64> {
        self.origin +
            Vector3::new(
                self.start_x + self.pixel_size * x as f64,
                self.start_y - self.pixel_size * y as f64,
                0f64,
            )
    }
}
