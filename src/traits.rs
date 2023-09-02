use crate::interval::Interval;
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
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, hit_record: &mut HitRecord) -> bool;
}
