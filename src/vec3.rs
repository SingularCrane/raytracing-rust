use crate::utils::*;
use std::f64::consts::PI;
use std::fmt;
use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub a: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { a: [x, y, z] }
    }

    pub fn random() -> Vec3 {
        Vec3 {
            a: [random_f64(), random_f64(), random_f64()],
        }
    }

    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            a: [random_range(min, max), random_range(min, max), random_range(min, max)],
        }
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let v = Vec3::random_in_unit_sphere();
        if v.dot(*normal) > 0.0 {
            v
        } else {
            -v
        }
    }

    pub fn random_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3 {
                a: [random_range(-1., 1.), random_range(-1., 1.), 0.],
            };
            if p.length_squared() < 1. {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        Self::random_in_unit_sphere().unit_vector()
        // Self::random().unit_vector()
    }

    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_f64();
        let r2 = random_f64();
        let z = 1.0 + r2 * ((1.0 - radius * radius / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z * z).sqrt();
        let y = phi.sin() * (1.0 - z * z).sqrt();

        Vec3::new(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.a[0]
    }

    pub fn y(&self) -> f64 {
        self.a[1]
    }

    pub fn z(&self) -> f64 {
        self.a[2]
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
        (self.x().abs() < s) && (self.y().abs() < s) && (self.z().abs() < s)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(&self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }

    pub fn dot(&self, other: Vec3) -> f64 {
        self.x() * other.x() + self.y() * other.y() + self.z() * other.z()
    }

    pub fn cross(&self, other: Vec3) -> Vec3 {
        Vec3 {
            a: [
                self.a[1] * other.a[2] - self.a[2] * other.a[1],
                self.a[2] * other.a[0] - self.a[0] * other.a[2],
                self.a[0] * other.a[1] - self.a[1] * other.a[0],
            ],
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
            a: [-self.x(), -self.y(), -self.z()],
        }
    }
}

// + Vec3
impl ops::Add<Vec3> for Vec3 {
    type Output = Self;
    fn add(self, _rhs: Vec3) -> Self::Output {
        Self {
            a: [self.x() + _rhs.x(), self.y() + _rhs.y(), self.z() + _rhs.z()],
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
            a: [self.x() * _rhs, self.y() * _rhs, self.z() * _rhs],
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
            a: [self.x() * _rhs.x(), self.y() * _rhs.y(), self.z() * _rhs.z()],
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
        self.a[0] += _rhs.a[0];
        self.a[1] += _rhs.a[1];
        self.a[2] += _rhs.a[2];
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
        self.a[0] *= _rhs;
        self.a[1] *= _rhs;
        self.a[2] *= _rhs;
    }
}

// /=
impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, _rhs: f64) {
        self.a[0] /= _rhs;
        self.a[1] /= _rhs;
        self.a[2] /= _rhs;
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, i: usize) -> &Self::Output {
        &self.a[i]
    }
}

impl ops::IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.a[i]
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(x:{} y:{} z:{})", self.x(), self.y(), self.z())
    }
}
