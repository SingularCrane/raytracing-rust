use crate::ray::*;

#[derive(Copy, Clone)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn new(min: Point3, max: Point3) -> AABB {
        AABB { min: min, max: max }
    }

    pub fn surrounding_box(box0: AABB, box1: AABB) -> AABB {
        let small = Point3::new(
            box0.min.x().min(box1.min.x()),
            box0.min.y().min(box1.min.y()),
            box0.min.z().min(box1.min.z()),
        );
        let big = Point3::new(
            box0.max.x().max(box1.max.x()),
            box0.max.y().max(box1.max.y()),
            box0.max.z().max(box1.max.z()),
        );
        AABB::new(small, big)
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        let mut max = t_max;
        let mut min = t_min;
        for a in 0..3 {
            let t0 = ((self.min.a[a] - r.orig.a[a]) / r.dir.a[a])
                .min((self.max.a[a] - r.orig.a[a]) / r.dir.a[a]);
            let t1 = ((self.min.a[a] - r.orig.a[a]) / r.dir.a[a])
                .max((self.max.a[a] - r.orig.a[a]) / r.dir.a[a]);
            min = min.min(t0);
            max = max.max(t1);
            if max <= min {
                return false;
            }
        }
        true
    }
}
