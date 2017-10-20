extern crate image;

use self::image::{ImageBuffer, Pixel, Rgba};
use std::path::Path;
use std::io;

use color::Color;

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

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.image_buffer.put_pixel(
            x as u32,
            y as u32,
            color.to_rgba(),
        );
    }

    pub fn clear_pixel(&mut self, x: usize, y: usize) {
        self.image_buffer.put_pixel(
            x as u32,
            y as u32,
            Rgba::from_channels(0, 0, 0, 255),
        );
    }

    pub fn clear_image_buffer(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.clear_pixel(x, y);
            }
        }
    }

    pub fn save_image(&mut self, filename: &str) -> io::Result<()> {
        self.image_buffer.save(Path::new(filename))
    }
}
