extern crate cgmath;

mod node;
pub mod bounding_box;

use std::f64;
use std::usize;

use self::cgmath::*;
use self::node::Node;
use self::bounding_box::BoundingBox;
use ray_tracer::intersection::Intersection;
use ray_tracer::object::*;
use ray_tracer::ray::Ray;

pub struct KdTree {
    nodes: Vec<Node>,
    next_index: usize,
    root_index: usize,
    max_depth: usize,
}

impl KdTree {
    fn scene_bounding_box(objects: &[Shape]) -> BoundingBox {
        let mut min: Vector3<f64> = Vector3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max: Vector3<f64> = Vector3::new(f64::MIN, f64::MIN, f64::MIN);

        // Determine initial bounding box for entire scene
        for shape in objects.iter() {
            let shape_min: Vector3<f64> = shape.min_extent();

            if shape_min[0] < min[0] {
                min[0] = shape_min[0];
            }
            if shape_min[1] < min[1] {
                min[1] = shape_min[1];
            }
            if shape_min[2] < min[2] {
                min[2] = shape_min[2];
            }

            let shape_max: Vector3<f64> = shape.max_extent();

            if shape_max[0] > max[0] {
                max[0] = shape_max[0];
            }
            if shape_max[1] > max[1] {
                max[1] = shape_max[1];
            }
            if shape_max[2] > max[2] {
                max[2] = shape_max[2];
            }
        }

        BoundingBox::new(min, max)
    }

    fn new_node(
        &mut self,
        bounding_box: BoundingBox,
        objects: Vec<Shape>,
        split_axis: usize,
        attempt: usize,
        depth: usize,
    ) -> usize {
        // Average midpoint of objects
        let midpoint: f64 = Node::midpoint(&objects, split_axis);

        // Objects for each side: (lt_objects, gt_objects)
        let split_objects: (Vec<Shape>, Vec<Shape>) = Node::split(&objects, midpoint, split_axis);

        if depth > self.max_depth {
            // Artificial depth limit
            self.add_node(bounding_box, objects, split_axis, None, None)
        } else if split_objects.0.len() == objects.len() || split_objects.1.len() == objects.len() {
            // Base case: split has no worthwhile effect - one side still has all objects

            // Tried all three axes, give up
            if attempt > 3 {
                return self.add_node(bounding_box, objects, split_axis, None, None);
            }

            // Try again on a different axis
            self.new_node(
                bounding_box,
                objects,
                (split_axis + 1) % 3,
                attempt + 1,
                depth + 1,
            )
        } else {
            // Split on longest axis
            let next_split_axis: usize = bounding_box.largest_axis();

            let mut lt_max: Vector3<f64> = bounding_box.max;
            lt_max[split_axis] = midpoint;
            let lt_bounding_box: BoundingBox = BoundingBox::new(bounding_box.min, lt_max);

            let mut gt_min: Vector3<f64> = bounding_box.min;
            gt_min[split_axis] = midpoint;
            let gt_bounding_box: BoundingBox = BoundingBox::new(gt_min, bounding_box.max);

            let lt_node_id: usize = self.new_node(
                lt_bounding_box,
                split_objects.0,
                next_split_axis,
                1,
                depth + 1,
            );

            let gt_node_id: usize = self.new_node(
                gt_bounding_box,
                split_objects.1,
                next_split_axis,
                1,
                depth + 1,
            );

            self.add_node(
                bounding_box,
                Vec::new(), // No need to save objects in non-leaf node
                split_axis,
                Some(lt_node_id),
                Some(gt_node_id),
            )
        }
    }

    pub fn add_node(
        &mut self,
        bounding_box: BoundingBox,
        objects: Vec<Shape>,
        split_axis: usize,
        lt_node_id: Option<usize>,
        gt_node_id: Option<usize>,
    ) -> usize {
        let node_id: usize = self.next_index;

        self.nodes.push(Node::new(
            node_id,
            bounding_box,
            objects,
            split_axis,
            lt_node_id,
            gt_node_id,
        ));

        self.next_index += 1;

        node_id
    }

    pub fn root_node(&self) -> &Node {
        &self.nodes[self.root_index]
    }

    fn traverse(&self, ray: &Ray, node: &Node) -> Option<Intersection> {
        if node.is_leaf() {
            // Leaf node - perform actual object intersection tests
            return Intersection::closest_intersection(ray, &node.objects);
        }

        let lt_node: &Node = &self.nodes[node.lt_node_id.unwrap()];
        let gt_node: &Node = &self.nodes[node.gt_node_id.unwrap()];

        let lt_intersection: bool = lt_node.bounding_box.intersect_test(ray);

        if lt_intersection && gt_node.bounding_box.intersect_test(ray) {
            // Intersects both sides

            let lt_intersection: Option<Intersection> = self.traverse(ray, lt_node);
            let gt_intersection: Option<Intersection> = self.traverse(ray, gt_node);

            if lt_intersection.is_none() {
                // No object intersections on lt side
                gt_intersection
            } else if gt_intersection.is_none() {
                // No object intersections on gt side
                lt_intersection
            } else {
                // Intersections on both sides - choose closest

                let lt_intersection: Intersection = lt_intersection.unwrap();
                let gt_intersection: Intersection = gt_intersection.unwrap();

                if lt_intersection.distance < gt_intersection.distance {
                    Some(lt_intersection)
                } else {
                    Some(gt_intersection)
                }
            }
        } else if lt_intersection {
            // LT Intersection only
            self.traverse(ray, lt_node)
        } else {
            // GT Intersection only
            self.traverse(ray, gt_node)
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let root_node: &Node = &self.nodes[self.root_index];

        if !root_node.bounding_box.intersect_test(ray) {
            // Ray doesn't intersect scene bounding box
            return None;
        }

        self.traverse(ray, root_node)
    }

    pub fn new(objects: &[Shape], max_depth: usize) -> KdTree {
        // Copy objects - do not take ownership
        let mut objects = objects.to_vec();

        let mut tree = KdTree {
            nodes: Vec::new(),
            next_index: 0,
            root_index: 0,
            max_depth,
        };

        tree.root_index = tree.new_node(KdTree::scene_bounding_box(&objects), objects, 0, 1, 0);

        tree
    }
}
