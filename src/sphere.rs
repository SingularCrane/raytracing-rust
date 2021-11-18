use crate::hittable::*;
use crate::material::*;
use crate::ray::*;
use std::rc::Rc;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            center: c,
            radius: r,
            material: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center;
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), root, self.material.clone());
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Rc<dyn Material>,
}

impl MovingSphere {
    pub fn new(
        cen0: Point3,
        cen1: Point3,
        time0: f64,
        time1: f64,
        radius: f64,
        material: Rc<dyn Material>,
    ) -> MovingSphere {
        MovingSphere {
            center0: cen0,
            center1: cen1,
            time0: time0,
            time1: time1,
            radius: radius,
            material: material,
        }
    }

    fn center(&self, time: f64) -> Point3 {
        self.center0
            - ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.orig - self.center(r.time);
        let a = r.dir.length_squared();
        let half_b = oc.dot(r.dir);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0. {
            return None;
        }
        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let mut rec = HitRecord::new(r.at(root), root, self.material.clone());
        let outward_normal = (rec.p - self.center(r.time)) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }
}
