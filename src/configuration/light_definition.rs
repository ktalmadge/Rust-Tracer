extern crate cgmath;

extern crate serde;
extern crate serde_json;

use color::Color;
use light::Light;

#[derive(Serialize, Deserialize)]
pub struct LightDefinition {
    position: Vec<f64>,
    luminosity: f64,
    color: Vec<u8>,
}

impl LightDefinition {
    pub fn as_light(&self) -> Light {
        Light::new(
            ::Configuration::parse_vector(&self.position),
            self.luminosity,
            self.parsed_color(),
        )
    }

    fn parsed_color(&self) -> Color {
        Color::new(
            *(self.color.get(0).unwrap()) as f64,
            *(self.color.get(1).unwrap()) as f64,
            *(self.color.get(2).unwrap()) as f64,
        )
    }
}
