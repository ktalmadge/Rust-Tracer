extern crate cgmath;

extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::prelude::*;

use cgmath::Vector3;

mod object_definition;
mod light_definition;

use self::object_definition::ObjectDefinition;
use self::light_definition::LightDefinition;

use camera::Camera;

#[derive(Serialize, Deserialize)]
pub struct Configuration {
    pub threads: usize,
    pub width: usize,
    pub height: usize,
    pub camera_position: Vec<f64>,
    pub camera_target: Vec<f64>,
    pub viewport_distance: f64,
    pub viewport_width: f64,
    pub ambient_coefficient: f64,
    pub specular_coefficient: f64,
    pub diffuse_coefficient: f64,
    pub specular_exponent: f64,
    pub max_reflections: u8,
    pub reinhard_key_value: f64,
    pub reinhard_delta: f64,
    pub objects: Vec<ObjectDefinition>,
    pub lights: Vec<LightDefinition>,
}

impl Configuration {
    fn parse_vector(vector: &[f64]) -> Vector3<f64> {
        Vector3::new(
            f64::from(vector[0]),
            f64::from(vector[1]),
            f64::from(vector[2]),
        )
    }

    pub fn read_configuration(filename: &str) -> Configuration {
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        serde_json::from_str(&contents).unwrap()
    }

    pub fn camera(&self) -> Camera {
        Camera::new(
            Configuration::parse_vector(&self.camera_position),
            Configuration::parse_vector(&self.camera_target),
        )
    }
}
