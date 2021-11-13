use crate::vec3::Vec3;

pub type Point3 = Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
}

pub trait At {
    fn at(&self, t: f64) -> Point3;
}

impl At for Ray {
    fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
