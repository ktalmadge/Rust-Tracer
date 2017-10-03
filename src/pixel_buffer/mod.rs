extern crate image;

use self::image::{ImageBuffer, Pixel, Rgba};
use std::path::Path;
use std::io;

pub struct PixelBuffer {
    width: usize,
    height: usize,
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> PixelBuffer {
        let mut pb = PixelBuffer {
            width,
            height,
            image_buffer: ImageBuffer::new(width as u32, height as u32),
        };

        pb.clear_image_buffer();

        pb
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8, a: u8) {
        self.image_buffer.put_pixel(
            x as u32,
            y as u32,
            Rgba::from_channels(r, g, b, a),
        );
    }

    pub fn set_pixel_rgba(&mut self, x: usize, y: usize, color: Rgba<u8>) {
        self.image_buffer.put_pixel(x as u32, y as u32, color);
    }

    pub fn clear_image_buffer(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x, y, 0, 0, 0, 255);
            }
        }
    }

    pub fn save_image(&mut self, filename: &str) -> io::Result<()> {
        self.image_buffer.save(Path::new(filename))
    }
}
