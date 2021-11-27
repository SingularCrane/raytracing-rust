use rand::prelude::*;

pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}

pub fn random_int(min: usize, max: usize) -> usize {
    random_range(min as f64, (max + 1) as f64).floor() as usize
}
