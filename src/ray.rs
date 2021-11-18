use crate::vec3::Vec3;

pub type Point3 = Vec3;

pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(orig: Point3, dir: Vec3, time: f64) -> Ray {
        Ray {
            orig: orig,
            dir: dir,
            time: time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
