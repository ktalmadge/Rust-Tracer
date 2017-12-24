extern crate cgmath;
use self::cgmath::*;

use ray_tracer::ray::Ray;

use std::f64;

#[derive(Debug)]
pub struct BoundingBox {
    pub min: Vector3<f64>,
    pub max: Vector3<f64>,
}

impl BoundingBox {
    pub fn largest_axis(&self) -> usize {
        let x_size: f64 = self.max[0] - self.min[0];
        let y_size: f64 = self.max[1] - self.min[1];
        let z_size: f64 = self.max[2] - self.min[2];

        if x_size > y_size {
            if x_size > z_size { 0 } else { 2 }
        } else if y_size > z_size {
            1
        } else {
            2
        }
    }

    // Intersection without point
    pub fn intersect_test(&self, ray: &Ray) -> bool {
        let mut tmin: f64 = f64::MIN;
        let mut tmax: f64 = f64::MAX;

        for i in 0..3 {
            let t1: f64 = (self.min[i] - ray.origin[i]) * ray.inv_dir[i];
            let t2: f64 = (self.max[i] - ray.origin[i]) * ray.inv_dir[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        tmax >= tmin.max(0f64)
    }

    // Intersection with point
    pub fn intersect(&self, ray: &Ray) -> Option<Vector3<f64>> {
        let mut tmin: f64 = f64::MIN;
        let mut tmax: f64 = f64::MAX;

        for i in 0..3 {
            let t1: f64 = (self.min[i] - ray.origin[i]) * ray.inv_dir[i];
            let t2: f64 = (self.max[i] - ray.origin[i]) * ray.inv_dir[i];

            tmin = tmin.max(t1.min(t2));
            tmax = tmax.min(t1.max(t2));
        }

        if tmax < tmin.max(0f64) {
            return None;
        }

        let intersection: Vector3<f64> = if tmin < 0f64 {
            // Ray originates inside box
            ray.direction * tmax + ray.origin
        } else {
            ray.direction * tmin + ray.origin
        };

        Some(intersection)
    }

    pub fn new(min: Vector3<f64>, max: Vector3<f64>) -> BoundingBox {
        BoundingBox { min, max }
    }
}
