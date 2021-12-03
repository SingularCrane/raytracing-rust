use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::random_f64;
use crate::vec3::Vec3;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable>,
    phase_function: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(boundary: Arc<dyn Hittable>, phase_function: Arc<dyn Material>, density: f64) -> ConstantMedium {
        ConstantMedium {
            boundary: boundary,
            phase_function: phase_function,
            neg_inv_density: -1.0 / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let enable_debug = false;
        let debugging = enable_debug && random_f64() < 0.00001;

        if let Some(mut rec1) = self.boundary.hit(r, -f64::INFINITY, f64::INFINITY) {
            if let Some(mut rec2) = self.boundary.hit(r, rec1.t + 0.0001, f64::INFINITY) {
                if debugging {
                    eprint!("\nt_min= {}, t_max = {}", rec1.t, rec2.t)
                }
                if rec1.t < t_min {
                    rec1.t = t_min
                }
                if rec2.t > t_max {
                    rec2.t = t_max
                }
                if rec1.t >= rec2.t {
                    return None;
                }
                let ray_length = r.dir.length();
                let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_f64().log10();
                if hit_distance > distance_inside_boundary {
                    return None;
                }
                let t = rec1.t + hit_distance / ray_length;
                let mut rec = HitRecord::new(r.at(t), t, 0.0, 0.0, self.phase_function.clone());
                rec.front_face = true;
                rec.normal = Vec3::new(0.0, 0.0, 1.0);
                if debugging {
                    eprint!("hit_distance = {}\nrec.t = {}, rec.p = {}\n", hit_distance, rec.t, rec.p);
                }
                Some(rec)
            } else {
                None
            }
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}
