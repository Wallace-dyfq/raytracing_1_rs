use crate::interval::Interval;
use crate::HitRecord;
use crate::Hittable;
use crate::Point3;
use crate::Ray;
use crate::Scatter;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    material: Rc<dyn Scatter>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, material: Rc<dyn Scatter>) -> Self {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord> {
        let oc = &ray.orig - &self.center;
        let a = ray.dir.dot(&ray.dir);
        let half_b = oc.dot(&ray.dir);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        } else {
            // find the nearest root that lies in the acceptable range
            let mut root = (-half_b - discriminant.sqrt()) / a;
            if !ray_t.surrounds(root) {
                root = (-half_b + discriminant.sqrt()) / a;
                if !ray_t.surrounds(root) {
                    return None;
                }
            }
            let mut hit_record = HitRecord::new();
            hit_record.t = root;
            hit_record.point = ray.at(root);
            let outward_normal = (&hit_record.point - &self.center) / self.radius;
            hit_record.set_face_normal(&ray, &outward_normal);
            hit_record.material = self.material.clone();
            Some(hit_record)
        }
    }
}
