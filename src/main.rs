mod camera;
mod color;
mod hittables;
mod interval;
mod material;
mod ray;
mod sphere;
mod traits;
mod utils;
mod vec3;
use std::env;

use camera::Camera;
use color::write_color;
use color::Color;
use hittables::{HitRecord, Hittables};
use interval::Interval;
use material::{Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use traits::{Hittable, Scatter};
use vec3::{unit_vector, Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() {
    let args: Vec<String> = env::args().collect();
    //meterial
    let material_ground = Rc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Rc::new(Lambertian {
        albedo: Color::new(0.7, 0.3, 0.3),
    });
    let material_left = Rc::new(Metal {
        albedo: Color::new(0.8, 0.8, 0.8),
    });
    let material_right = Rc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
    });
    // world
    let world: Hittables = Hittables {
        objects: vec![
            Box::new(Sphere::new(
                Point3::new(0.0, -100.5, -1.0),
                100.0,
                material_ground.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(0.0, 0.0, -1.0),
                0.5,
                material_center.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(-1.0, 0.0, -1.0),
                0.5,
                material_left.clone(),
            )),
            Box::new(Sphere::new(
                Point3::new(1.0, 0.0, -1.0),
                0.5,
                material_right.clone(),
            )),
        ],
    };

    // camera
    let camera = Camera::new(16.0 / 9.0, 400, 50, 50);
    camera.render(&world);
}
