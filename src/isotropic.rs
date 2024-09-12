use crate::hittable::HitRecord;
use crate::material::{Material, ScatterPDF, ScatterRecord};
use crate::ray::Ray3;
use crate::solid_texture::SolidTexture;
use crate::sphere_pdf::SpherePDF;
use crate::texture::Texture;
use crate::vector_3::Vec3;

use std::f64::consts::FRAC_1_PI;
use std::sync::Arc;
pub struct Isotropic {
    albedo: Arc<dyn Texture + Sync + Send>,
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _ray_in: &crate::ray::Ray3,
        hit_rec: &HitRecord,
        scatter_rec: &mut ScatterRecord,
    ) -> bool {
        scatter_rec.attenuation = self.albedo.value(hit_rec.u, hit_rec.v, hit_rec.point);
        scatter_rec.pdf = ScatterPDF::PDF(Box::new(SpherePDF {}));
        true
    }

    fn scattering_pdf(&self, _ray_in: &Ray3, _hit_record: &HitRecord, _scattered: &Ray3) -> f64 {
        0.25 * FRAC_1_PI
    }
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture + Sync + Send>) -> Isotropic {
        Isotropic { albedo }
    }

    pub fn color(color: Vec3) -> Isotropic {
        Isotropic::new(Arc::new(SolidTexture::new(color)))
    }
}
