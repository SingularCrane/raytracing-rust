use crate::ray::*;
use crate::utils::*;
use crate::vec3::Vec3;

const POINT_COUNT: usize = 256;

pub struct Perlin {
    ranvec: Vec<Vec3>,
    perm_x: Vec<usize>,
    perm_y: Vec<usize>,
    perm_z: Vec<usize>,
}

impl Perlin {
    pub fn new() -> Perlin {
        let mut p = Perlin {
            ranvec: vec![Vec3::new(0., 0., 0.); POINT_COUNT],
            perm_x: vec![0; POINT_COUNT],
            perm_y: vec![0; POINT_COUNT],
            perm_z: vec![0; POINT_COUNT],
        };
        for i in 0..POINT_COUNT {
            p.ranvec[i] = Vec3::random().unit_vector();
            p.perm_x[i] = i;
            p.perm_y[i] = i;
            p.perm_z[i] = i;
        }
        for i in (0..POINT_COUNT).rev() {
            let t_x = random_int(0, i);
            let t_y = random_int(0, i);
            let t_z = random_int(0, i);
            let mut tmp = p.perm_x[i];
            p.perm_x[i] = p.perm_x[t_x];
            p.perm_x[t_x] = tmp;
            tmp = p.perm_y[i];
            p.perm_y[i] = p.perm_y[t_y];
            p.perm_y[t_y] = tmp;
            tmp = p.perm_z[i];
            p.perm_z[i] = p.perm_z[t_z];
            p.perm_z[t_z] = tmp;
        }
        p
    }

    pub fn noise(&self, p: Point3) -> f64 {
        let mut u = p.x() - p.x().floor();
        let mut v = p.y() - p.y().floor();
        let mut w = p.z() - p.z().floor();
        // u = u * u * (3. - 2. * u);
        // v = v * v * (3. - 2. * v);
        // w = w * w * (3. - 2. * w);

        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;

        let mut c: [[[Vec3; 2]; 2]; 2] = [[[Vec3::new(0., 0., 0.); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let x = ((i + di as i32) & 255) as usize;
                    let y = ((j + dj as i32) & 255) as usize;
                    let z = ((k + dk as i32) & 255) as usize;
                    c[di][dj][dk] = self.ranvec[self.perm_x[x] ^ self.perm_y[y] ^ self.perm_z[z]];
                }
            }
        }
        trilinear_interp(c, u, v, w)
    }

    pub fn turb(&self, p: Point3, depth: usize) -> f64 {
        let mut accum: f64 = 0.;
        let mut temp_p = p;
        let mut weight = 1.;

        for _ in 0..depth {
            accum += weight * self.noise(temp_p);
            weight *= 0.5;
            temp_p *= 2.;
        }
        accum.abs()
    }
}

fn trilinear_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
    let uu = u * u * (3. - 2. * u);
    let vv = v * v * (3. - 2. * v);
    let ww = w * w * (3. - 2. * w);
    let mut accum = 0.;
    for i in 0..2 {
        for j in 0..2 {
            for k in 0..2 {
                let fi = i as f64;
                let fj = j as f64;
                let fk = k as f64;
                let weight_v = Vec3::new(u - fi, v - fj, w - fk);
                accum += (fi * u + (1. - fi) * (1. - u))
                    * (fj * v + (1. - fj) * (1. - v))
                    * (fk * w + (1. - fk) * (1. - w))
                    * c[i][j][k].dot(weight_v);
            }
        }
    }
    accum
}
