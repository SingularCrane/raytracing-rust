use crate::aabb::*;
use crate::hittable::*;
use crate::material::*;
use crate::prelude::Vec3;
use crate::prelude::ONB;
use crate::ray::*;
use std::sync::Arc;

use std::f64::consts::PI;

pub struct Sphere {
    center: Point3,
    radius: f64,
    material: Arc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Arc<dyn Material>) -> Sphere {
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
        let outward_normal = (r.at(root) - self.center) / self.radius;
        let (u, v) = get_uv(outward_normal);
        let mut rec = HitRecord::new(r.at(root), root, u, v, self.material.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<Aabb> {
        Some(Aabb::new(
            self.center - Point3::new(self.radius, self.radius, self.radius),
            self.center + Point3::new(self.radius, self.radius, self.radius),
        ))
    }

    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if let Some(_rec) = self.hit(&Ray::new(*o, *v, 0.0), 0.001, f64::INFINITY) {
            let cos_theta_max = (1.0 - self.radius * self.radius / (self.center - *o).length_squared()).sqrt();
            let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, o: &Vec3) -> Vec3 {
        let dir = self.center - *o;
        let distance_squared = dir.length_squared();
        let uvw = ONB::build_from_w(&dir);
        uvw.local_vec(&Vec3::random_to_sphere(self.radius, distance_squared))
    }
}

pub struct MovingSphere {
    center0: Point3,
    center1: Point3,
    time0: f64,
    time1: f64,
    radius: f64,
    material: Arc<dyn Material>,
}

impl MovingSphere {
    pub fn new(cen0: Point3, cen1: Point3, time0: f64, time1: f64, radius: f64, material: Arc<dyn Material>) -> MovingSphere {
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
        self.center0 - ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
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

        let outward_normal = (r.at(root) - self.center(r.time)) / self.radius;
        let (u, v) = get_uv(outward_normal);
        let mut rec = HitRecord::new(r.at(root), root, u, v, self.material.clone());
        rec.set_face_normal(r, &outward_normal);
        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<Aabb> {
        let box0 = Aabb::new(
            self.center(time0) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time0) + Point3::new(self.radius, self.radius, self.radius),
        );
        let box1 = Aabb::new(
            self.center(time1) - Point3::new(self.radius, self.radius, self.radius),
            self.center(time1) + Point3::new(self.radius, self.radius, self.radius),
        );
        Some(Aabb::surrounding_box(box0, box1))
    }
}

fn get_uv(p: Point3) -> (f64, f64) {
    let theta = (-p.y()).acos();
    let phi = (-p.z()).atan2(p.x()) + PI;

    (phi / (2. * PI), theta / PI)
}
