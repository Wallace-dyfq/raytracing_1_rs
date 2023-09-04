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
use material::{Dielectric, Lambertian, Metal};
use ray::Ray;
use sphere::Sphere;
use std::fs::File;
use std::io::BufWriter;
use std::rc::Rc;
use traits::{Hittable, Scatter};
use vec3::{Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let arg1 = env::args().nth(1);
    let output_fname = if let Some(fname) = arg1 {
        fname
    } else {
        "images/image_0.ppm".to_string()
    };
    let file = File::create(output_fname)?;
    let mut writer = BufWriter::new(file);
    let mut world = Hittables::default();
    //meterial
    let material_ground = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground.clone(),
    )));

    let count = 11;
    let p = Point3::new(4.0, 0.2, 0.0);
    for a in -count..count {
        for b in -count..count {
            let choose_mat = utils::random_f64();
            let center = Point3::new(
                a as f64 + utils::random_f64(),
                0.2,
                b as f64 + utils::random_f64(),
            );

            if (&center - &p).length() > 0.9 {
                match choose_mat {
                    x if x < 0.8 => {
                        // difuse
                        let sphere_material = Rc::new(Lambertian::new(
                            &Color::random(0.0, 1.0) * &Color::random(0.0, 1.0),
                        ));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                    }
                    x if x < 0.95 => {
                        // matel
                        let albedo = Color::random(0.5, 1.0);
                        let fuzz = utils::random_f64_range(0.0, 0.5);
                        let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                    }
                    _ => {
                        // glass
                        let sphere_material = Rc::new(Dielectric::new(1.5));
                        world.add(Box::new(Sphere::new(center, 0.2, sphere_material.clone())));
                    }
                };
            }
        }
    }

    let material_1 = Rc::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material_1.clone(),
    )));
    let material_2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material_2.clone(),
    )));
    let material_3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material_3.clone(),
    )));

    // camera
    let mut camera = Camera::new(
        16.0 / 9.0,
        1200,  /* image width*/
        500,  /* sample per pixel */
        50,   /* max depth */
        20.0, /* vfov */
    );
    camera.look_from = Point3::new(13.0, 2.0, 3.0);
    camera.look_at = Point3::new(0.0, 0.0, 0.0);
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    if let Ok(()) = camera.render(&world, &mut writer) {
        println!("Program runs Ok");
    } else {
        eprintln!("Program runs NOT Ok");
    }
    Ok(())
}
