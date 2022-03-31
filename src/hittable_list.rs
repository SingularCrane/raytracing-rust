use crate::prelude::{random_int, Aabb, HitRecord, Hittable, Point3, Ray, Vec3};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: Vec::new() }
    }

    pub fn new_from_list(object: Arc<dyn Hittable>) -> HittableList {
        HittableList { objects: vec![object] }
    }

    pub fn add(&mut self, to_add: Arc<dyn Hittable>) {
        self.objects.push(to_add);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut temp_rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if let Some(rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                temp_rec = Some(rec);
            }
        }
        temp_rec
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if self.objects.len() == 0 {
            return None;
        }

        let mut output_box = Aabb::new(Point3::new(0., 0., 0.), Point3::new(0., 0., 0.));
        let mut first_box = true;
        for object in self.objects.iter() {
            if let Some(temp_box) = object.bounding_box(time0, time1) {
                output_box = if first_box {
                    temp_box
                } else {
                    Aabb::surrounding_box(output_box, temp_box)
                };
                first_box = false;
            }
        }
        Some(output_box)
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(o, v);
        }

        sum
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        self.objects[random_int(0, self.objects.len() - 1)].random(o)
    }
}
