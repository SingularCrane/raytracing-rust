use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::{Point3, Ray};
use crate::utils::random_range;
use crate::vec3::Vec3;

use std::sync::Arc;

pub struct XYRect {
    mat: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl Hittable for XYRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.z()) / r.dir.z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x() + t * r.dir.x();
        let y = r.orig.y() + t * r.dir.y();
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (y - self.y0) / (self.y1 - self.y0);
        let outward_normal = Vec3::new(0., 0., 1.);
        let mut rec = HitRecord::new(r.at(t), t, u, v, self.mat.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.y0, self.k - 0.0001),
            Point3::new(self.x0, self.y0, self.k + 0.0001),
        ))
    }
}

impl XYRect {
    pub fn new(x0: f64, x1: f64, y0: f64, y1: f64, k: f64, mat: Arc<dyn Material>) -> XYRect {
        XYRect {
            x0: x0,
            x1: x1,
            y0: y0,
            y1: y1,
            k: k,
            mat: mat,
        }
    }
}

pub struct XZRect {
    mat: Arc<dyn Material>,
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.y()) / r.dir.y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.orig.x() + t * r.dir.x();
        let z = r.orig.z() + t * r.dir.z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (x - self.x0) / (self.x1 - self.x0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(0., 1., 0.);
        let mut rec = HitRecord::new(r.at(t), t, u, v, self.mat.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.x0, self.k - 0.0001, self.z0),
            Point3::new(self.x0, self.k + 0.0001, self.z1),
        ))
    }
    fn pdf_value(&self, origin: &Point3, v: &Vec3) -> f64 {
        if let Some(rec) = self.hit(&Ray::new(*origin, *v, 0.0), 0.001, f64::INFINITY) {
            let area = (self.x1 - self.x0) * (self.z1 - self.z0);
            let distance_squared = rec.t * rec.t * v.length_squared();
            let cosine = v.dot(rec.normal).abs() / v.length();

            distance_squared / (cosine * area)
        } else {
            0.0
        }
    }
    fn random(&self, origin: &Point3) -> Vec3 {
        let random_point = Point3::new(random_range(self.x0, self.x1), self.k, random_range(self.z0, self.z1));
        random_point - *origin
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> XZRect {
        XZRect {
            x0: x0,
            x1: x1,
            z0: z0,
            z1: z1,
            k: k,
            mat: mat,
        }
    }
}

pub struct YZRect {
    mat: Arc<dyn Material>,
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.orig.x()) / r.dir.x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.orig.y() + t * r.dir.y();
        let z = r.orig.z() + t * r.dir.z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }
        let u = (y - self.y0) / (self.y1 - self.y0);
        let v = (z - self.z0) / (self.z1 - self.z0);
        let outward_normal = Vec3::new(1., 0., 0.);
        let mut rec = HitRecord::new(r.at(t), t, u, v, self.mat.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            Point3::new(self.k - 0.0001, self.y0, self.z0),
            Point3::new(self.k + 0.0001, self.y0, self.z0),
        ))
    }
}

impl YZRect {
    pub fn new(y0: f64, y1: f64, z0: f64, z1: f64, k: f64, mat: Arc<dyn Material>) -> YZRect {
        YZRect {
            y0: y0,
            y1: y1,
            z0: z0,
            z1: z1,
            k: k,
            mat: mat,
        }
    }
}
