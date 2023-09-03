use crate::interval::Interval;
use crate::Color;
use crate::HitRecord;
use crate::Point3;
use crate::Ray;
use crate::Vec3;
pub trait Hittable {
    fn hit(&self, ray: &Ray, ray_t: &mut Interval, hit_record: &mut HitRecord) -> bool;
}
