use crate::Color;
use crate::Ray;
use crate::Scatter;
use crate::Vec3;
use crate::utils::random_f64;

// diffusive
#[derive(Default, Debug, Clone)]
pub struct Lambertian {
    albedo: Color,
}

// reflective
#[derive(Default, Debug, Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[derive(Default, Debug, Clone)]
pub struct Dielectric {
    ir: f64, // index of reflection
}

impl Lambertian {
    pub fn new(color: Color) -> Self {
        Self { albedo: color }
    }
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
        attenuation.set_with_other(&self.albedo);
        true
    }
}

impl Metal {
    pub fn new(color: Color, fuzz: f64) -> Self {
        Self {
            albedo: color,
            fuzz,
        }
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
            dir: reflected + Vec3::random_unit_vec3() * self.fuzz,
        };
        attenuation.set_with_other(&self.albedo);
        true
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        // use Schlick's approximation for reflectance
        let r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        let r0 = r0.powi(2);
        r0 + (1.0-r0)*(1.0 - cosine).powi(5)

    }
}

impl Scatter for Dielectric {
    fn scatter(
        &self,
        ray_in: &Ray,
        rec: &crate::hittables::HitRecord,
        attenuation: &mut Color,
        ray_scattered: &mut Ray,
    ) -> bool {
        attenuation.set(1.0, 1.0, 1.0);

        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = ray_in.dir.make_unit_vector();
        let cos_theta = unit_direction.reverse().dot(&rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        let can_reflect = refraction_ratio * sin_theta > 1.0;
        let direction = if can_reflect || Self::reflectance(cos_theta, refraction_ratio) > random_f64()  {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };
        *ray_scattered = Ray {
            orig: rec.point.clone(),
            dir: direction,
        };

        true
    }
}
