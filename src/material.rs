use crate::color::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Color,
}

impl Scatter {
    pub fn new(ray: Ray, attenuation: Color) -> Scatter {
        Scatter {
            ray: ray,
            attenuation: attenuation,
        }
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(c: Color) -> Lambertian {
        Lambertian { albedo: c }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        return Some(Scatter::new(
            Ray::new(rec.p, scatter_direction),
            self.albedo,
        ));
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(c: Color, f: f64) -> Metal {
        Metal { albedo: c, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = r_in.dir.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_unit_sphere());
        if scattered.dir.dot(rec.normal) > 0. {
            return Some(Scatter::new(scattered, self.albedo));
        } else {
            return None;
        }
    }
}
