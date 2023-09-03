use crate::utils::*;
use crate::write_color;
use crate::{unit_vector, Color, HitRecord, Hittable, Hittables, Interval, Point3, Ray, Vec3};
#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,       // number of pixels
    pub samples_per_pixel: u32, // number of pixels
    image_height: u32,
    center: Point3, // Camera center
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
}

impl Camera {
    pub fn new(aspect_ratio: f64, image_width: u32, samples_per_pixel: u32) -> Self {
        let mut camera = Camera::default();
        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.samples_per_pixel = samples_per_pixel;
        camera.initialize();
        camera
    }

    pub fn render(&self, world: &Hittables) {
        println!("P3\n{} {}\n255", self.image_width, self.image_height);
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center = &self.pixel00_loc
                    + (&self.pixel_delta_u * i as f64)
                    + (&self.pixel_delta_v * j as f64);

                let ray_direction = &pixel_center - &self.center;
                let ray = Ray {
                    orig: self.center.clone(),
                    dir: ray_direction,
                };
                let mut pixel_color = Color::default();
                for sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += &self.ray_color(&r, world);
                }

                let _ = write_color(&mut std::io::stdout(), &pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let pixel_center =
            &self.pixel00_loc + (&self.pixel_delta_u * i as f64) + (&self.pixel_delta_v * j as f64);
        let pixel_sample = &pixel_center + self.pixel_sample_square();
        let ray_origin = self.center.clone();
        let ray_direction = &pixel_sample - &ray_origin;
        return Ray {
            orig: ray_origin,
            dir: ray_direction,
        };
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        return (&self.pixel_delta_u * px) + (&self.pixel_delta_v * py);
    }

    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as u32).max(1);

        self.center = Point3::new(0.0, 0.0, 0.0);

        let viewport_height = 2.0;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        let focal_length = 1.0;
        // calculate the vector across the horizontal and down the vertical view edge
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // calculate the location of upper left pixel
        let viewport_upper_left = &self.center
            - Vec3::new(0.0, 0.0, focal_length)
            - &viewport_u / 2.0
            - &viewport_v / 2.0;

        self.pixel00_loc = &viewport_upper_left + (&self.pixel_delta_u + &self.pixel_delta_v) * 0.5;
    }

    fn ray_color(&self, ray: &Ray, hittables: &Hittables) -> Color {
        let mut rec = HitRecord::default();
        if hittables.hit(&ray, &mut Interval::new(0.0, INFINITY), &mut rec) {
            let direction = Vec3::random_unit_on_hemisphere(&rec.normal);
            let new_ray = Ray {
                orig: rec.point.clone(),
                dir: direction,
            };

            return self.ray_color(&new_ray, hittables) * 0.5;
            //return (rec.normal + Color::new(1.0, 1.0, 1.0)) * 0.5;
        }
        let unit_direction = unit_vector(&ray.dir);
        let a = 0.5 * (unit_direction.y() + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
