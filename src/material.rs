use crate::color::*;
use crate::hittable::*;
use crate::onb::ONB;
use crate::pdf::random_cosine_direction;
use crate::prelude::CosinePDF;
use crate::prelude::PDF;
use crate::ray::*;
use crate::texture::*;
use crate::utils::*;
use crate::vec3::*;
use std::f64::consts::PI;
use std::sync::Arc;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord>;
    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
    fn emitted(&self, _rec: &HitRecord, _u: f64, _v: f64, _p: Point3) -> Color {
        Color::new(0., 0., 0.)
    }
}

pub struct ScatterRecord {
    pub ray: Ray,
    pub attenuation: Color,
    pub pdf: Option<Arc<dyn PDF>>,
}

impl ScatterRecord {
    pub fn new(ray: Ray, attenuation: Color, pdf: Option<Arc<dyn PDF>>) -> ScatterRecord {
        ScatterRecord { ray, attenuation, pdf }
    }
}

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(c: Color) -> Lambertian {
        Lambertian::new_textured(Arc::new(SolidColor::new(c)))
    }
    pub fn new_textured(t: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: t.clone() }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let uvw = ONB::build_from_w(&rec.normal);
        let direction = uvw.local_vec(&random_cosine_direction());
        Some(ScatterRecord::new(
            Ray::new(rec.p, direction.unit_vector(), r_in.time),
            self.albedo.value(rec.u, rec.v, rec.p),
            Some(Arc::new(CosinePDF::new(&rec.normal))),
        ))
    }
    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = rec.normal.dot(scattered.dir.unit_vector());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let reflected = r_in.dir.unit_vector().reflect(rec.normal);
        let scattered = Ray::new(rec.p, reflected + self.fuzz * Vec3::random_in_unit_sphere(), r_in.time);
        if scattered.dir.dot(rec.normal) > 0. {
            return Some(ScatterRecord::new(scattered, self.albedo, None));
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
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
        return r0 + (1. - r0) * (1. - cosine).powf(5.);
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        let attenuation = Color::new(1., 1., 1.);
        let refraction_ratio = if rec.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = (-unit_direction.dot(rec.normal)).min(1.);
        let sin_theta = (1. - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.;
        let direction = if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > random_f64() {
            unit_direction.reflect(rec.normal)
        } else {
            unit_direction.refract(rec.normal, refraction_ratio)
        };
        return Some(ScatterRecord::new(Ray::new(rec.p, direction, r_in.time), attenuation, None));
    }
}

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<ScatterRecord> {
        None
    }

    fn emitted(&self, rec: &HitRecord, u: f64, v: f64, p: Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Color::new(0.0, 0.0, 0.0)
        }
    }
}

impl DiffuseLight {
    pub fn new_textured(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit: emit }
    }

    pub fn new_color(emit_color: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(emit_color)),
        }
    }
}

// pub struct Isotropic {
//     albedo: Arc<dyn Texture>,
// }

// impl Isotropic {
//     pub fn new_color(c: Color) -> Isotropic {
//         Isotropic {
//             albedo: Arc::new(SolidColor::new(c)),
//         }
//     }

//     pub fn new_textured(t: Arc<dyn Texture>) -> Isotropic {
//         Isotropic { albedo: t }
//     }
// }

// impl Material for Isotropic {
//     fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<Scatter> {
//         Some(Scatter::new(
//             Ray::new(rec.p, Vec3::random_unit_sphere(), r_in.time),
//             self.albedo.value(rec.u, rec.v, rec.p),
//         ))
//     }
// }
