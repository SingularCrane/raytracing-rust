use crate::color::*;
use crate::hittable::*;
use crate::ray::*;
use crate::utils::*;
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        return Some(Scatter::new(
            Ray::new(rec.p, scatter_direction, r_in.time),
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
        let scattered = Ray::new(
            rec.p,
            reflected + self.fuzz * Vec3::random_unit_sphere(),
            r_in.time,
        );
        if scattered.dir.dot(rec.normal) > 0. {
            return Some(Scatter::new(scattered, self.albedo));
        } else {
            return None;
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir: ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
        return r0 + (1. - r0) * (1. - cosine).powf(5.);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64()
        {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };
        return Some(Scatter::new(
            Ray::new(rec.p, direction, r_in.time),
            attenuation,
        ));
    }
}
