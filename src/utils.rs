pub const INFINITY: f64 = f64::MAX;
pub const PI: f64 = std::f64::consts::PI;

pub fn degrees_to_radians(d: f64) -> f64 {
    d * PI / 180.0
}
pub fn radians_to_degrees(r: f64) -> f64 {
    r * 180.0 / PI
}
