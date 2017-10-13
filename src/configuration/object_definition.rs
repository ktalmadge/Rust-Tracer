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
    pub fn read_objects(&self) -> Vec<Box<::object::Object>> {
        let mut r: Reader = Reader::new();
        r.read_file(&(self.filename)).unwrap();
        r.objects
    }

    fn parsed_color(&self) -> Color {
        Color::new(
            *(self.color.get(0).unwrap()) as f64,
            *(self.color.get(1).unwrap()) as f64,
            *(self.color.get(2).unwrap()) as f64,
        )
    }
}
