mod color;
mod ray;
mod vec3;
use std::env;

use color::write_color;
use color::Color;
use ray::Ray;
use vec3::{unit_vector, Point3, Vec3};
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn hit_sphere(center: &Point3, radius: f64, ray: &Ray) -> f64 {
    let oc = &ray.orig - center;
    let a = ray.dir.dot(&ray.dir);
    let half_b = oc.dot(&ray.dir);
    let c = oc.dot(&oc) - radius * radius;
    let discriminant = half_b * half_b - a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        let result = (-half_b - discriminant.sqrt()) / a;
        return result;
    }
}

// return all black for now
fn ray_color(ray: &ray::Ray) -> Color {
    // sphere parameters
    let center = Point3::new(0.0, 0.0, -1.0);
    let radius = 0.5;
    let t = hit_sphere(&center, radius, ray);
    if t > 0.0 {
        let tmp = &ray.at(t) - &center;
        let n = unit_vector(&tmp);
        return Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0) * 0.5;
    }
    let unit_direction = unit_vector(&ray.dir);
    let a = 0.5 * (unit_direction.y() + 1.0);

    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    // calculate the image height, and ensure that it is at least 1
    let image_height = ((image_width as f64 / aspect_ratio) as u32).max(1);

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let focal_length = 1.0;
    // calculate the vector across the horizontal and down the vertical view edge
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // calculate the location of upper left pixel
    let viewport_upper_left =
        &camera_center - &Vec3::new(0.0, 0.0, focal_length) - &viewport_u / 2.0 - &viewport_v / 2.0;

    let pixel00_loc = &viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;
    println!("P3\n{} {}\n255", image_width, image_height);
    //    eprintln!("aspect_ratio: {}, viewport_height: {}, viewport_width: {}\nviewport_u: {}, viewport_v: {}\npixel_delta_u:{}, pixel_delta_v:{}\nview_port_upper_left: {}, pixel00_loc: {}", aspect_ratio, viewport_height, viewport_width, viewport_u, viewport_v, pixel_delta_u, pixel_delta_v, viewport_upper_left, pixel00_loc);

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                &pixel00_loc + (&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);

            let ray_direction = &pixel_center - &camera_center;
            let ray = Ray {
                orig: camera_center.clone(),
                dir: ray_direction,
            };
            let pixel_color = ray_color(&ray);

            let _ = write_color(&mut std::io::stdout(), &pixel_color);
        }
    }
}
