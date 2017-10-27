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
mod ray;
mod reader;
mod scene;
mod tone;

use color::Color;
use scene::configuration::Configuration;
use scene::Scene;

use std::thread;
use std::path::Path;

use self::image::{ImageBuffer, Rgba};

pub fn save_image(filename: &str, color_buffer: &Vec<Vec<Color>>, width: usize, height: usize) {
    let mut image_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::new(width as u32, height as u32);

    for x in 0..width {
        for y in 0..height {
            image_buffer.put_pixel(x as u32, y as u32, color_buffer[x][y].to_rgba());
        }
    }

    image_buffer.save(Path::new(filename)).unwrap();
}

pub fn combine_scenes(
    color_buffer: &mut Vec<Vec<Color>>,
    scenes: Vec<(usize, Scene)>,
    threads: usize,
) {
    for (thread_number, scene) in scenes {
        for (x, y) in scene.draw_iterator(threads, thread_number) {
            color_buffer[x][y] = scene.get_pixel(x, y);
        }
    }
}

pub fn draw(config_file: &str, out_file: &str) {
    let configuration: Configuration = Configuration::read_configuration(config_file);

    let threads: usize = configuration.threads;
    let width: usize = configuration.width;
    let height: usize = configuration.height;

    let mut thread_handles: Vec<thread::JoinHandle<_>> = Vec::with_capacity(threads);
    for i in 0..threads {
        let mut scene: Scene = Scene::new(&configuration);

        thread_handles.push(thread::spawn(move || {
            scene.partial_draw(threads, i);
            (i, scene)
        }));
    }

    // Collect results from each thread into one color buffer
    let mut scenes: Vec<(usize, Scene)> = Vec::with_capacity(threads);
    for thread_handle in thread_handles {
        scenes.push(thread_handle.join().unwrap());
    }

    let mut color_buffer: Vec<Vec<Color>> = vec![vec![Color::new(0f64, 0f64, 0f64); height]; width];
    combine_scenes(&mut color_buffer, scenes, threads);

    // Tone correction
    tone::reinhard_tone_correction(
        &mut color_buffer,
        configuration.width as usize,
        configuration.height as usize,
        configuration.reinhard_key_value,
        configuration.reinhard_delta,
    );

    // Save the image
    save_image(
        out_file,
        &color_buffer,
        configuration.width,
        configuration.height,
    );
}

fn main() {
    draw("./configuration.json", "img/scene.png");
}
