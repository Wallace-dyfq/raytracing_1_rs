use crate::utils::*;
use crate::write_color;
use crate::Scatter;
use crate::{Color, HitRecord, Hittable, Hittables, Interval, Point3, Ray, Vec3};
#[derive(Debug, Default)]
pub struct Camera {
    pub aspect_ratio: f64,
    pub image_width: u32,       // number of pixels
    pub samples_per_pixel: u32, // number of pixels
    pub max_depth: i32,         // maximum number of ray bounces into scene
    pub vfov: f64,              // vertical view angle (field of view)
    pub look_from: Point3,      // point camera is looking from
    pub look_at: Point3,        // point camera is looking at
    pub vup: Vec3,              // camera-relative "up" direction
    pub defocus_angle: f64,     // variation angle of rays through each pixel
    pub focus_dist: f64,        // distance from camera look from point to plane of perfect focus
    image_height: u32,
    center: Point3, // Camera center
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3, // camera frame basis vector
    v: Vec3, // camera frame basis vector
    w: Vec3, // camera frame basis vector
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
        vfof: f64,
    ) -> Self {
        let mut camera = Camera::default();
        camera.aspect_ratio = aspect_ratio;
        camera.image_width = image_width;
        camera.samples_per_pixel = samples_per_pixel;
        camera.max_depth = max_depth;
        camera.vfov = vfof;
        camera.look_from = Point3::new(0.0, 0.0, 0.0);
        camera.look_at = Point3::new(0.0, 0.0, -1.0);
        camera.vup = Vec3::new(0.0, 1.0, 0.0);
        camera.defocus_angle = 10.0;
        camera.focus_dist = 3.4;
        camera
    }

    pub fn render(&mut self, world: &Hittables) {
        self.initialize();
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
                    pixel_color += &self.ray_color(&r, self.max_depth, world);
                }

                let _ = write_color(&mut std::io::stdout(), &pixel_color, self.samples_per_pixel);
            }
        }
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // get a randomly-sampled camera ray for the pixel at location i,j, originating from the
        // camera defocus disk
        let pixel_center =
            &self.pixel00_loc + (&self.pixel_delta_u * i as f64) + (&self.pixel_delta_v * j as f64);
        let pixel_sample = &pixel_center + self.pixel_sample_square();
        let ray_origin = if self.defocus_angle < 0.0 {
            self.center.clone()
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = &pixel_sample - &ray_origin;
        return Ray {
            orig: ray_origin,
            dir: ray_direction,
        };
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        &self.center + (p.x() * &self.defocus_disk_u) + (p.y() * &self.defocus_disk_v)
    }

    fn pixel_sample_square(&self) -> Vec3 {
        let px = -0.5 + random_f64();
        let py = -0.5 + random_f64();
        return (&self.pixel_delta_u * px) + (&self.pixel_delta_v * py);
    }

    // initialize some internal state
    fn initialize(&mut self) {
        self.image_height = ((self.image_width as f64 / self.aspect_ratio) as u32).max(1);

        // determine viewport dimensions
        self.center = self.look_from.clone();
        let theta = degrees_to_radians(self.vfov);
        let h = f64::tan(theta / 2.0);

        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);
        // calculate u,v,w unit basis vector for the camera coordinate frame
        self.w = (&self.look_from - &self.look_at).make_unit_vector();
        self.u = self.vup.cross(&self.w);
        self.v = self.w.cross(&self.u);

        // calculate the vector across the horizontal and down the vertical view edge
        let viewport_u = viewport_width * &self.u;
        let viewport_v = -viewport_height * &self.v;

        // calculate the horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = &viewport_u / self.image_width as f64;
        self.pixel_delta_v = &viewport_v / self.image_height as f64;

        // calculate the location of upper left pixel
        let viewport_upper_left =
            &self.center - (self.focus_dist * &self.w) - &viewport_u / 2.0 - &viewport_v / 2.0;

        self.pixel00_loc = &viewport_upper_left + (&self.pixel_delta_u + &self.pixel_delta_v) * 0.5;
        let defofus_radius =
            self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = &self.u * defofus_radius;
        self.defocus_disk_v = &self.v * defofus_radius;
    }

    fn ray_color(&self, ray: &Ray, depth: i32, hittables: &Hittables) -> Color {
        if depth == 0 {
            return Color::default();
        }
        if let Some(rec) = hittables.hit(&ray, &mut Interval::new(0.001, INFINITY)) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            if rec
                .material
                .scatter(&ray, &rec, &mut attenuation, &mut scattered)
            {
                return &attenuation * &self.ray_color(&scattered, depth - 1, &hittables);
            }
            return Color::default();
        }
        let unit_direction = Vec3::unit_vector(&ray.dir);
        let a = 0.5 * (unit_direction.y() + 1.0);

        Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
    }
}
