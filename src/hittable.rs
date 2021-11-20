use crate::material::*;
use crate::ray::*;
use crate::vec3::*;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, material: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p: p,
            normal: Vec3::new(0., 0., 0.),
            t: t,
            front_face: false,
            mat: material,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.dir.dot(*outward_normal) < 0.;
        if self.front_face {
            self.normal = *outward_normal;
        } else {
            self.normal = -*outward_normal;
        }
    }
}
