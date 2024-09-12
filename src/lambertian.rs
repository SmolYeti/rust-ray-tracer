use crate::cosine_pdf::CosinePDF;
use crate::hittable::HitRecord;
use crate::material::{Material, ScatterPDF, ScatterRecord};
use crate::ray::Ray3;
use crate::solid_texture::SolidTexture;
use crate::texture::Texture;
use crate::vector_3::Vec3;
use std::f64::consts::FRAC_1_PI;
use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture + Sync + Send>,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _ray_in: &Ray3,
        hit_rec: &HitRecord,
        scatter_rec: &mut ScatterRecord,
    ) -> bool {
        scatter_rec.attenuation = self.albedo.value(hit_rec.u, hit_rec.v, hit_rec.point);
        scatter_rec.pdf = ScatterPDF::PDF(Box::new(CosinePDF::new(&hit_rec.normal)));
        true
    }

    fn scattering_pdf(&self, _ray_in: &Ray3, hit_rec: &HitRecord, scattered: &Ray3) -> f64 {
        let cosine_theta = hit_rec.normal.dot(&scattered.direction().unit_vector());
        if cosine_theta < 0.0 {
            0.0
        } else {
            cosine_theta * FRAC_1_PI
        }
    }
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture + Sync + Send>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn from_color(color: Vec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidTexture::new(color)),
        }
    }
}
