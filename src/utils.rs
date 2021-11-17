use rand::prelude::*;

pub fn random_f64() -> f64 {
    let mut rng = thread_rng();
    rng.gen()
}

pub fn random_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
