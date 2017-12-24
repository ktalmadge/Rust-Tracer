use super::bounding_box::BoundingBox;
use ray_tracer::object::*;

pub struct Node {
    pub node_id: usize,
    pub lt_node_id: Option<usize>,
    pub gt_node_id: Option<usize>,
    pub objects: Vec<Shape>,
    pub bounding_box: BoundingBox,
    pub split_axis: usize,
}

impl Node {
    // Determine the average midpoint on the given axis
    pub fn midpoint(objects: &[Shape], split_axis: usize) -> f64 {
        objects.iter().fold(0f64, |midpoint, &object| {
            midpoint + object.midpoint()[split_axis]
        }) / objects.len() as f64
    }

    // Determine which objects are on either side of the split
    // Some objects may be on both sides
    pub fn split(objects: &[Shape], midpoint: f64, split_axis: usize) -> (Vec<Shape>, Vec<Shape>) {
        let mut lt_objects: Vec<Shape> = Vec::new();
        let mut gt_objects: Vec<Shape> = Vec::new();

        for shape in objects.iter() {
            if shape.min_extent()[split_axis] <= midpoint {
                lt_objects.push(*shape);
            }

            if shape.max_extent()[split_axis] > midpoint {
                gt_objects.push(*shape);
            }
        }

        (lt_objects, gt_objects)
    }

    pub fn is_leaf(&self) -> bool {
        self.lt_node_id.is_none()
    }

    pub fn new(
        node_id: usize,
        bounding_box: BoundingBox,
        objects: Vec<Shape>,
        split_axis: usize,
        lt_node_id: Option<usize>,
        gt_node_id: Option<usize>,
    ) -> Node {
        Node {
            node_id,
            lt_node_id,
            gt_node_id,
            objects,
            bounding_box,
            split_axis,
        }
    }
}
