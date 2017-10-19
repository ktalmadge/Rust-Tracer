#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;
extern crate image;

#[macro_use]
extern crate serde_derive;

mod camera;
mod color;
mod light;
mod object;
mod pixel_buffer;
mod ray;
mod reader;
mod scene;

const WIDTH: usize = 300;
const HEIGHT: usize = 300;

#[cfg(test)]
mod tests {
    use super::*;

    use super::cgmath::Vector3;
}
