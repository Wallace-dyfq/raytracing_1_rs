use crate::interval::Interval;
use crate::Color;
use crate::HitRecord;
use crate::Ray;
pub trait Hittable {
    //TODO: make it return Option of HitRecord
    fn hit(&self, ray: &Ray, ray_t: &mut Interval) -> Option<HitRecord>;
}

pub trait Scatter {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool;
}
