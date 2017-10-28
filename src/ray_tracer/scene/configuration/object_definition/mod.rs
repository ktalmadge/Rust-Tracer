extern crate cgmath;

extern crate serde;
extern crate serde_json;

use ray_tracer::color::Color;
use ray_tracer::object::Shape;
use ray_tracer::object::material::Material;
use ray_tracer::reader::Reader;

#[derive(Serialize, Deserialize)]
pub struct ObjectDefinition {
    pub filename: String,
    pub color: Vec<u8>,
    pub reflective: bool,
}

impl ObjectDefinition {
    pub fn read_shapes(&self) -> Vec<Shape> {
        let mut r: Reader = Reader::new();
        let material: Material = Material::new(self.parsed_color(), self.reflective);
        r.read_file(&(self.filename), material).unwrap();
        r.shapes
    }

    fn parsed_color(&self) -> Color {
        Color::from_rgb(self.color[0], self.color[1], self.color[2])
    }
}
