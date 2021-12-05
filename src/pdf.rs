use crate::hittable::Hittable;
use crate::onb::ONB;
use crate::ray::Point3;
use crate::utils::random_f64;
use crate::vec3::Vec3;
use std::f64::consts::PI;
use std::sync::Arc;
pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

pub fn random_cosine_direction() -> Vec3 {
    let r1 = random_f64();
    let r2 = random_f64();
    let z = (1.0 - r2).sqrt();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    Vec3 { a: [x, y, z] }
}
pub struct CosinePDF {
    uvw: ONB,
}

impl CosinePDF {
    pub fn new(w: &Vec3) -> CosinePDF {
        CosinePDF { uvw: ONB::build_from_w(w) }
    }
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = direction.unit_vector().dot(self.uvw.w());
        if cosine <= 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_vec(&random_cosine_direction())
    }
}

pub struct HittablePDF {
    o: Point3,
    h: Arc<dyn Hittable>,
}

impl HittablePDF {
    pub fn new(h: Arc<dyn Hittable>, o: Point3) -> HittablePDF {
        HittablePDF { o, h }
    }
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.h.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.h.random(&self.o)
    }
}

pub struct MixturePDF {
    p: [Arc<dyn PDF>; 2],
}

impl MixturePDF {
    pub fn new(p0: Arc<dyn PDF>, p1: Arc<dyn PDF>) -> MixturePDF {
        MixturePDF { p: [p0, p1] }
    }
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_f64() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
