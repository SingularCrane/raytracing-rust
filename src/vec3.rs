use crate::utils::*;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { x: x, y: y, z: z }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            x: random_f64(),
            y: random_f64(),
            z: random_f64(),
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            x: random_range(min, max),
            y: random_range(min, max),
            z: random_range(min, max),
        }
    }

    pub fn random_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random();
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_unit_sphere().unit_vector()
    }

    pub fn reflect(&self, normal: Vec3) -> Vec3 {
        return *self - 2. * self.dot(normal) * normal;
    }

    pub fn refract(&self, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let costheta = (-*self).dot(normal).min(1.0);
        let r_out_perp = etai_over_etat * (*self + costheta * normal);
        let r_out_parallel = -((1. - r_out_perp.length_squared()).abs().sqrt()) * normal;
        return r_out_perp + r_out_parallel;
    }

    pub fn near_zero(&self) -> bool {
        let s = 1.0e-8;
        (self.x < s) && (self.y < s) && (self.z < s)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.z * other.y - self.y * other.x,
        }
    }

    pub fn unit_vector(&self) -> Vec3 {
        let len = self.length();
        *self / len
    }
}

// Negate
impl ops::Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

// + Vec3
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> Self::Output {
        Self {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z,
        }
    }
}

// - Vec3
impl ops::Sub<Vec3> for Vec3 {
    type Output = Self;
    fn sub(self, _rhs: Vec3) -> Self::Output {
        self + (-_rhs)
    }
}

// multiplication
impl ops::Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, _rhs: f64) -> Self::Output {
        Self {
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        _rhs * self
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, _rhs: Vec3) -> Self::Output {
        Self {
            x: self.x * _rhs.x,
            y: self.y * _rhs.y,
            z: self.z * _rhs.z,
        }
    }
}

// / f64
impl ops::Div<f64> for Vec3 {
    type Output = Self;
    fn div(self, _rhs: f64) -> Self::Output {
        (1.0 / _rhs) * self
    }
}

// += Vec3
impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
        self.z += _rhs.z;
    }
}

// -= Vec3
impl ops::SubAssign<Vec3> for Vec3 {
    fn sub_assign(&mut self, _rhs: Self) {
        *self += -_rhs;
    }
}

// *= f64
impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, _rhs: f64) {
        self.x *= _rhs;
        self.y *= _rhs;
        self.z *= _rhs;
    }
}

// /=
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        self.x /= _rhs;
        self.y /= _rhs;
        self.z /= _rhs;
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}
