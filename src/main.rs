mod camera;
mod color;
mod hittables;
mod interval;
mod ray;
mod sphere;
mod traits;
mod utils;
mod vec3;
use std::env;

use camera::Camera;
use color::write_color;
use color::Color;
use hittables::Hittables;
use interval::Interval;
use ray::Ray;
use sphere::Sphere;
use traits::{HitRecord, Hittable};
use vec3::{unit_vector, Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = env::args().collect();
    // world
    let world: Hittables = Hittables {
        objects: vec![
            Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)),
        ],
    };

    // camera
    let camera = Camera::new(16.0 / 9.0, 400);
    camera.render(&world);
}
