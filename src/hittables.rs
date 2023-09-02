use crate::traits::{HitRecord, Hittable};
use crate::Point3;
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
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, hit_record: &mut HitRecord) -> bool {
        let tmp_hit_record = &mut HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = ray_tmax;
        for object in self.objects.iter() {
            if object.hit(ray, ray_tmin, closest_so_far, tmp_hit_record) {
                hit_anything = true;
                closest_so_far = tmp_hit_record.t.clone();
                *hit_record = tmp_hit_record.clone();
            }
        }
        hit_anything
    }
}
