#![allow(dead_code)]
#![allow(unused_variables, unused_mut)]

extern crate cgmath;
extern crate image;

mod camera;
mod color;
mod light;
mod object;
mod pixel_buffer;
mod ray;
mod reader;
mod scene;

const WIDTH: usize = 640;
const HEIGHT: usize = 400;

use object::Object;
use cgmath::Vector3;

fn main() {
    /*  Set up objects */
    let mut sphere1: Box<Object> = Box::new(object::sphere::Sphere::new(
        Vector3::new(2.5f64, 1f64, -1f64),
        0.5f64,
    ));

    /* Set up lights */
    let mut light1: Box<light::Light> = Box::new(light::Light::new(
        Vector3::new(10f64, 10f64, 10f64),
        1f64,
        color::Color::new(255f64, 255f64, 255f64),
    ));

    let mut lights: Vec<Box<light::Light>> = Vec::new();
    lights.push(light1);


    /* Initiate and draw scene */
    let mut r: reader::Reader = reader::Reader::new();
    assert!(r.read_file("./test/icosahedron.obj").is_ok());

    r.objects.push(sphere1);

    let mut scene: scene::Scene = scene::Scene::new(WIDTH, HEIGHT, lights, r.objects, 0.4f64);
    scene.draw();
}
