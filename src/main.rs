
mod pixel_buffer;

use pixel_buffer::PixelBuffer;

const SIZE_X: usize = 640;
const SIZE_Y: usize = 400;
const BUFFER_SIZE: usize = SIZE_X * SIZE_Y * 3;

fn main() {
    let mut buffer: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

    let mut buffer: PixelBuffer = pixel_buffer::PixelBuffer::new(SIZE_X, SIZE_Y, &mut buffer);

    buffer.set_pixel(200, 100, 255, 0, 0);

    buffer.write_buffer("img/buff_out.png");
}
