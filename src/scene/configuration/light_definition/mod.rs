extern crate cgmath;

extern crate serde;
extern crate serde_json;

use color::Color;
use light::Light;

#[derive(Serialize, Deserialize)]
pub struct LightDefinition {
    position: Vec<f64>,
    intensity: f64,
    color: Vec<u8>,
}

impl LightDefinition {
    pub fn as_light(&self) -> Light {
        Light::new(
            super::Configuration::parse_vector(&self.position),
            self.intensity,
            self.parsed_color(),
        )
    }

    fn parsed_color(&self) -> Color {
        Color::from_rgb(self.color[0], self.color[1], self.color[2])
    }
}
