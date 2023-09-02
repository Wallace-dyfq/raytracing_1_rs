use rand::prelude::*;
pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(d: f64) -> f64 {
    d * PI / 180.0
}
pub fn radians_to_degrees(r: f64) -> f64 {
    r * 180.0 / PI
}
// generate random number between 0 and 1
pub fn random_f64() -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen(); // generates a float between 0 and 1
    y
}

// generate random number between given min and max
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    let mut rng = rand::thread_rng();
    let y: f64 = rng.gen_range(min..max); // generates a float between min and max
    y
}
