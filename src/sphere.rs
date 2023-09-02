use crate::interval::Interval;
use crate::traits::{HitRecord, Hittable};
use crate::Point3;
use crate::Ray;

#[derive(Debug, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64) -> Self {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let oc = &ray.orig - &self.center;
        let a = ray.dir.dot(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        } else {
            // find the nearest root that lies in the acceptable range
            let mut root = (-half_b - discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                root = (-half_b + discriminant.sqrt()) / a;
                if !ray_t.surrounds(root) {
                    return false;
                }
            }
            hit_record.t = root;
            hit_record.point = ray.at(root);
            let outward_normal = (&hit_record.point - &self.center) / self.radius;
            hit_record.set_face_normal(&ray, &outward_normal);
            return true;
        }
    }
}
