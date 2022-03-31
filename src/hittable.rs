use crate::aabb::Aabb;
use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::utils::degrees_to_radians;
use crate::vec3::Vec3;
use std::sync::Arc;

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb>;
    fn pdf_value(&self, _o: &Point3, _v: &Vec3) -> f64 {
        0.0
    }
    fn random(&self, _o: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, u: f64, v: f64, mat: Arc<dyn Material>) -> HitRecord {
        HitRecord {
            p,
            normal: Vec3::new(0., 0., 0.),
            t,
            u,
            v,
            front_face: false,
            mat,
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

pub struct Translate {
    h: Arc<dyn Hittable>,
    offset: Vec3,
}

impl Translate {
    pub fn new(h: Arc<dyn Hittable>, offset: Vec3) -> Translate {
        Translate { h: h, offset: offset }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_r = Ray::new(r.orig - self.offset, r.dir, r.time);
        if let Some(mut rec) = self.h.hit(&moved_r, t_min, t_max) {
            let normal = rec.normal;
            rec.p += self.offset;
            rec.set_face_normal(&moved_r, &normal);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        if let Some(out_box) = self.h.bounding_box(time0, time1) {
            Some(Aabb::new(out_box.min + self.offset, out_box.max + self.offset))
        } else {
            None
        }
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.h.pdf_value(o, v)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.h.random(o)
    }
}

pub struct RotateY {
    h: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<Aabb>,
}

impl RotateY {
    pub fn new(h: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = h.bounding_box(0.0, 1.0).unwrap();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max.x() + (1.0 - i as f64) * bbox.min.x();
                    let y = j as f64 * bbox.max.y() + (1.0 - j as f64) * bbox.min.y();
                    let z = k as f64 * bbox.max.z() + (1.0 - k as f64) * bbox.min.z();

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }
        RotateY {
            h,
            sin_theta,
            cos_theta,
            bbox: Some(Aabb::new(min, max)),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.orig;
        let mut dir = r.dir;

        origin[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        dir[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        dir[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_r = Ray::new(origin, dir, r.time);

        if let Some(mut rec) = self.h.hit(&rotated_r, t_min, t_max) {
            let mut p = rec.p;
            let mut normal = rec.normal;

            p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
            p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

            normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
            normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

            rec.p = p;
            rec.set_face_normal(&rotated_r, &normal);
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        self.bbox
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.h.pdf_value(o, v)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.h.random(o)
    }
}

pub struct FlipFace {
    h: Arc<dyn Hittable>,
}

impl FlipFace {
    pub fn new(h: Arc<dyn Hittable>) -> FlipFace {
        FlipFace { h: h }
    }
}

impl Hittable for FlipFace {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if let Some(mut rec) = self.h.hit(r, t_min, t_max) {
            rec.front_face = !rec.front_face;
            Some(rec)
        } else {
            None
        }
    }
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        self.h.bounding_box(time0, time1)
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        self.h.pdf_value(o, v)
    }
    fn random(&self, o: &Vec3) -> Vec3 {
        self.h.random(o)
    }
}
