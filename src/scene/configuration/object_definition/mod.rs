extern crate cgmath;

extern crate serde;
extern crate serde_json;

use color::Color;
use object::material::Material;
use reader::Reader;

#[derive(Serialize, Deserialize)]
pub struct ObjectDefinition {
    pub filename: String,
    pub color: Vec<u8>,
    pub reflective: bool,
}

impl ObjectDefinition {
    pub fn read_shapes(&self) -> Vec<::object::Shape> {
        let mut r: Reader = Reader::new();
        let material: Material = Material::new(self.parsed_color(), self.reflective);
        r.read_file(&(self.filename), material).unwrap();
        r.shapes
    }

    fn parsed_color(&self) -> Color {
        Color::new(
            f64::from(self.color[0]),
            f64::from(self.color[1]),
            f64::from(self.color[2]),
        )
    }
}
