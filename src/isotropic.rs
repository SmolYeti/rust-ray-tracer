use crate::{
    material::Material, ray::Ray3, solid_texture::SolidTexture, texture::Texture, vector_3::Vec3,
};
use std::sync::Arc;
pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(
        &self,
        ray_in: &crate::ray::Ray3,
        hit_record: &crate::hittable::HitRecord,
        attenuation: &mut crate::vector_3::Vec3,
        scattered: &mut crate::ray::Ray3,
    ) -> bool {
        *scattered = Ray3::new(hit_record.point, Vec3::random_unit_vector(), ray_in.time());
        *attenuation = self
            .albedo
            .value(hit_record.u, hit_record.v, hit_record.point);
        true
    }
}

impl Isotropic {
    pub fn new(albedo: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo }
    }

    pub fn color(color: Vec3) -> Isotropic {
        Isotropic::new(Arc::new(SolidTexture::new(color)))
    }
}
