use crate::interval::Interval;
use crate::traits::{HitRecord, Hittable};
use crate::Ray;

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
