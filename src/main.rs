extern crate cgmath;

mod pixel_buffer;
mod camera;

use pixel_buffer::PixelBuffer;
use self::cgmath::*;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

fn main() {
    let mut buffer: PixelBuffer = pixel_buffer::PixelBuffer::new(WIDTH, HEIGHT);

    buffer.set_pixel(200, 100, 255, 0, 0, 255);

    buffer.save_image("img/buff_out_vec.png").unwrap();

    let mut camera = camera::Camera::new(
        Vector3::new(0f64, 1f64, 0f64),
        Vector3::new(0f64, 0f64, 0f64),
    );

    println!("Dot test: {:?}", camera.dot_test());
}
