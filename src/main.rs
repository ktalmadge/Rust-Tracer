#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;

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

use scene::Scene;

fn main() {

    let mut scene: Scene = Scene::new("./configuration.json".to_string());
    scene.draw();
}
