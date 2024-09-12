use crate::hittable::HitRecord;
use crate::material::{Material, ScatterPDF, ScatterRecord};
use crate::ray::Ray3;
use crate::vector_3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray3, hit_rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        scatter_rec.attenuation = Vec3::new(1.0, 1.0, 1.0);

        let refraction_ratio = if hit_rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_dir = ray_in.direction().unit_vector();
        let cos_theta = (-unit_dir).dot(&hit_rec.normal).min(1.0);
        let sin_theta = (1.0 - (cos_theta * cos_theta)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction: Vec3;
        if cannot_refract || Dielectric::reflectance(cos_theta, refraction_ratio) > rand::random() {
            direction = unit_dir.reflect(&hit_rec.normal);
        } else {
            direction = Vec3::refract(&unit_dir, &hit_rec.normal, refraction_ratio);
        }

        let ray_out = Ray3::new(hit_rec.point, direction, ray_in.time());
        scatter_rec.pdf = ScatterPDF::Skip(ray_out);
        true
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }

    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + ((1.0 - r0) * (1.0 - cosine).powf(5.0))
    }
}
