extern crate cgmath;

extern crate serde;
extern crate serde_json;

use color::Color;
use reader::Reader;

#[derive(Serialize, Deserialize)]
pub struct ObjectDefinition {
    pub filename: String,
    pub color: Vec<u8>,
}

impl ObjectDefinition {
    pub fn read_shapes(&self) -> Vec<::object::Shape> {
        let mut r: Reader = Reader::new();
        r.read_file(&(self.filename), self.parsed_color()).unwrap();
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
