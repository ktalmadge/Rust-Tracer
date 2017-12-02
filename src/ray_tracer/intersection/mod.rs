extern crate cgmath;
use self::cgmath::*;

use std::f64;

use ray_tracer::object::*;
use ray_tracer::ray::Ray;


pub struct Intersection {
    pub ray_direction: Vector3<f64>,
    pub shape: Shape,
    pub point: Vector3<f64>,
    pub normal: Vector3<f64>,
    pub distance: f64,
}

impl Intersection {
    // Find the closest intersection from the ray's origin to an object
    pub fn closest_intersection(ray: &Ray, objects: &Vec<Shape>) -> Option<Intersection> {
        let mut result: Option<Intersection> = None;
        let mut shortest_distance: f64 = f64::MAX;
        for shape in objects {
            if let Some(intersection) = shape.intersect(ray) {
                let distance: f64 = (intersection - ray.origin).magnitude();
                if shortest_distance > distance {
                    shortest_distance = distance;

                    let normal: Vector3<f64> = shape.normal(intersection, ray.direction);

                    result = Some(Intersection::new(
                        ray.direction,
                        *shape,
                        intersection,
                        normal,
                        distance,
                    ));
                }
            }
        }

        result
    }

    pub fn new(
        ray_direction: Vector3<f64>,
        shape: Shape,
        point: Vector3<f64>,
        normal: Vector3<f64>,
        distance: f64,
    ) -> Intersection {
        Intersection {
            ray_direction,
            shape,
            point,
            normal,
            distance,
        }
    }
}
