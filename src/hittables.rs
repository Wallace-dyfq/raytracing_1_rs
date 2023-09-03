use crate::interval::Interval;
use crate::traits::{Hittable, Scatter};
use crate::Point3;
use crate::Ray;
use crate::Vec3;

#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // set the hit record normal vector,
    // assuming outward_normal has unit length, i.e., it is normalized
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
        self.front_face = ray.dir.dot(&outward_normal) < 0.0;
        self.normal = match self.front_face {
            true => outward_normal.clone(),
            _ => outward_normal * -1.0,
        }
    }
}
#[derive(Default)]
pub struct Hittables {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittables {
    pub fn add(&mut self, obj: Box<dyn Hittable>) {
        self.objects.push(obj);
    }
}

impl Hittable for Hittables {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, hit_record: &mut HitRecord) -> bool {
        let tmp_hit_record = &mut HitRecord::default();
        let mut hit_anything = false;
        for object in self.objects.iter() {
            if object.hit(ray, ray_t, tmp_hit_record) {
                hit_anything = true;
                ray_t.max = tmp_hit_record.t.clone();
                *hit_record = tmp_hit_record.clone();
            }
        }
        hit_anything
    }
}
