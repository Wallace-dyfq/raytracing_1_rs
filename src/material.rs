use crate::Color;
use crate::Ray;
use crate::Scatter;
use crate::Vec3;

// diffusive
#[derive(Default, Debug, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

// reflective
#[derive(Default, Debug, Clone)]
pub struct Metal {
    pub albedo: Color,
}

impl Scatter for Lambertian {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray,
        rec: &crate::hittables::HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = &rec.normal + Vec3::random_unit_vec3();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal.clone();
        }
        *ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: scatter_direction,
        };
        *attenuation = self.albedo.clone();
        true
    }
}
impl Scatter for Metal {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &crate::hittables::HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&ray_in.dir, &rec.normal);
        *ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: reflected,
        };
        *attenuation = self.albedo.clone();
        true
    }
}
