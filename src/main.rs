mod color;
mod vec3;
use std::env;

use color::write_color;
use color::Color;
use vec3::Vec3;
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;
fn main() {
    let args: Vec<String> = env::args().collect();

    //let width: u32 = args[1].parse().unwrap_or(256_u32);
    let width: u32 = args
        .get(1)
        .unwrap_or(&"256".to_string())
        .parse()
        .unwrap_or(256_u32);
    let height: u32 = args
        .get(2)
        .unwrap_or(&"256".to_string())
        .parse()
        .unwrap_or(256_u32);
    println!("P3\n{} {}\n255", width, height);
    for i in 0..height {
        for j in 0..width {
            let r = i as f64 / (width - 1) as f64;
            let g = j as f64 / (width - 1) as f64;
            let b = 0.0;
            let _ = write_color(&mut std::io::stdout(), &Color::new(r, g, b));
        }
    }
}
