#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;

#[macro_use]
extern crate serde_derive;

mod camera;
mod configuration;
mod color;
mod light;
mod object;
mod pixel_buffer;
mod ray;
mod reader;
mod scene;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

use configuration::Configuration;

fn main() {
    let mut configuration: Configuration =
        Configuration::read_configuration("./configuration.json");

    /* Set up lights */
    let mut lights: Vec<Box<light::Light>> = Vec::new();
    for light_definition in configuration.lights.iter() {
        lights.push(Box::new(light_definition.as_light()));
    }

    /*  Set up objects */
    let mut objects: Vec<Box<::object::Object>> = Vec::new();
    for object_definition in configuration.objects.iter() {
        objects.append(&mut (object_definition.read_objects()));
    }

    let mut scene: scene::Scene = scene::Scene::new(
        WIDTH,
        HEIGHT,
        lights,
        configuration.camera(),
        objects,
        configuration.viewport_distance,
        configuration.viewport_width,
        configuration.ambient_coefficient,
    );

    scene.draw();
}
