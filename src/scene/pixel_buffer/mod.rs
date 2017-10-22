extern crate image;

use self::image::{ImageBuffer, Rgba};
use std::path::Path;
use std::io;

use color::Color;

pub struct PixelBuffer {
    width: usize,
    height: usize,
    color_buffer: Vec<Vec<Color>>,
    image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>>,
}

impl PixelBuffer {
    pub fn new(width: usize, height: usize) -> PixelBuffer {
        PixelBuffer {
            width,
            height,
            color_buffer: vec![vec![Color::new(0f64, 0f64, 0f64); height]; width],
            image_buffer: ImageBuffer::new(width as u32, height as u32),
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.color_buffer[x][y] = color;
    }

    pub fn save_image(&mut self, filename: &str) -> io::Result<()> {
        for x in 0..self.width {
            for y in 0..self.height {
                self.image_buffer.put_pixel(
                    x as u32,
                    y as u32,
                    self.color_buffer[x][y].to_rgba(),
                );
            }
        }

        self.image_buffer.save(Path::new(filename))
    }
}
