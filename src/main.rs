mod pixel_buffer;

use pixel_buffer::PixelBuffer;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

fn main() {
    let mut buffer: PixelBuffer = pixel_buffer::PixelBuffer::new(WIDTH, HEIGHT);

    buffer.set_pixel(200, 100, 255, 0, 0, 255);

    buffer.save_image("img/buff_out_vec.png").unwrap();
}
