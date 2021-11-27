use crate::aabb::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::ray::*;
use crate::utils::*;
use std::sync::Arc;

pub struct BVHNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(
        left: Arc<dyn Hittable>,
        right: Arc<dyn Hittable>,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        BVHNode {
            left: left.clone(),
            right: right.clone(),
            bbox: AABB::surrounding_box(
                left.bounding_box(time0, time1).unwrap(),
                right.bounding_box(time0, time1).unwrap(),
            ),
        }
    }

    pub fn new_from_list(objects: HittableList, time0: f64, time1: f64) -> BVHNode {
        return BVHNode::new_from_vec(
            objects.objects.clone(),
            0,
            objects.objects.len(),
            time0,
            time1,
        );
    }

    pub fn new_from_vec(
        src_objects: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BVHNode {
        let mut objects = src_objects[start..end].to_vec();
        let axis = random_int(0, 2);
        let comparator = if axis == 0 {
            box_x_compare
        } else if axis == 1 {
            box_y_compare
        } else {
            box_z_compare
        };
        let object_span = objects.len();

        if object_span == 1 {
            return BVHNode::new(objects[0].clone(), objects[0].clone(), time0, time1);
        } else if object_span == 2 {
            if comparator(objects[0].clone(), objects[1].clone()).is_gt() {
                return BVHNode::new(objects[0].clone(), objects[1].clone(), time0, time1);
            } else {
                return BVHNode::new(objects[1].clone(), objects[0].clone(), time0, time1);
            }
        } else {
            objects.sort_by(|a, b| comparator(a.clone(), b.clone()));
            let mid = object_span / 2;
            return BVHNode::new(
                Arc::new(BVHNode::new_from_vec(objects.clone(), 0, mid, time0, time1)),
                Arc::new(BVHNode::new_from_vec(
                    objects.clone(),
                    mid,
                    object_span,
                    time0,
                    time1,
                )),
                time0,
                time1,
            );
        }
    }
}

impl Hittable for BVHNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }
        let hit_left = self.left.hit(r, t_min, t_max);

        let hit_right = self.right.hit(
            r,
            t_min,
            if let Some(l) = hit_left.as_ref() {
                l.t
            } else {
                t_max
            },
        );
        hit_right.or(hit_left)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox)
    }
}

fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> std::cmp::Ordering {
    let box_a = a.bounding_box(0., 0.).unwrap();
    let box_b = b.bounding_box(0., 0.).unwrap();

    box_a.min.a[axis].partial_cmp(&box_b.min.a[axis]).unwrap()
}

fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> std::cmp::Ordering {
    box_compare(a, b, 2)
}
