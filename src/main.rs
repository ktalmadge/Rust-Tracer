#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;
extern crate image;

#[macro_use]
extern crate serde_derive;

mod ray_tracer;

fn main() {
    ray_tracer::draw("./configuration.json", "img/scene.png");
}
