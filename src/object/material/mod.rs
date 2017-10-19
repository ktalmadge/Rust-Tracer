use color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub reflective: bool,
}

impl Material {
    pub fn new(color: Color, reflective: bool) -> Material {
        Material { color, reflective }
    }
}
