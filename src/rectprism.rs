use crate::aabb::AABB;
use crate::aarect::{XYRect, XZRect, YZRect};
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::material::Material;
use crate::ray::{Point3, Ray};
use std::sync::Arc;

pub struct RectPrism {
    min: Point3,
    max: Point3,
    sides: HittableList,
}

impl RectPrism {
    pub fn new(p0: Point3, p1: Point3, mat: Arc<dyn Material>) -> RectPrism {
        let mut r = RectPrism {
            min: p0,
            max: p1,
            sides: HittableList::new(),
        };
        r.sides
            .add(Arc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p1.z(), mat.clone())));
        r.sides
            .add(Arc::new(XYRect::new(p0.x(), p1.x(), p0.y(), p1.y(), p0.z(), mat.clone())));
        r.sides
            .add(Arc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p1.y(), mat.clone())));
        r.sides
            .add(Arc::new(XZRect::new(p0.x(), p1.x(), p0.z(), p1.z(), p0.y(), mat.clone())));
        r.sides
            .add(Arc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p1.x(), mat.clone())));
        r.sides
            .add(Arc::new(YZRect::new(p0.y(), p1.y(), p0.z(), p1.z(), p0.x(), mat.clone())));
        r
    }
}

impl Hittable for RectPrism {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}
