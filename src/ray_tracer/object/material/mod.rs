use ray_tracer::color::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub normal: f64,
    pub reflectance: f64,
    pub ambient_coefficient: f64,
    pub diffuse_coefficient: f64,
    pub specular_coefficient: f64,
    pub specular_exponent: f64,
}

impl Material {
    pub fn new(
        color: Color,
        reflectance: f64,
        ambient_coefficient: f64,
        specular_coefficient: f64,
        specular_exponent: f64,
    ) -> Material {
        Material {
            color,
            normal: 1f64 - reflectance,
            reflectance,
            ambient_coefficient,
            specular_coefficient,
            specular_exponent,
            diffuse_coefficient: 1f64 - ambient_coefficient - specular_coefficient,
        }
    }
}
