extern crate image;

pub struct PixelBuffer<'a> {
    size_x: usize,
    size_y: usize,
    buffer: &'a mut [u8],
}

impl<'a> PixelBuffer<'a> {
    pub fn new(x: usize, y: usize, initial_buffer: &'a mut [u8]) -> PixelBuffer {
        PixelBuffer {
            size_x: x,
            size_y: y,
            buffer: initial_buffer,
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, r: u8, g: u8, b: u8) {
        // In english: Skip Y rows of SIZE_X, offset by X; 3 bytes per pixel
        let pixel_offset: usize = y * self.size_x * 3 + x * 3;

        self.buffer[pixel_offset] = r;
        self.buffer[pixel_offset + 1] = g;
        self.buffer[pixel_offset + 2] = b;
    }

    pub fn write_buffer(&mut self, filename: &str) {
        image::save_buffer(
            filename,
            self.buffer,
            self.size_x as u32,
            self.size_y as u32,
            image::RGB(8),
        ).unwrap();
    }
}
