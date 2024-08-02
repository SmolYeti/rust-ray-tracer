use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray3;
use crate::solid_texture::SolidTexture;
use crate::texture::Texture;
use crate::vector_3::Vec3;
use std::sync::Arc;

pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: &Ray3,
        _hit_record: &HitRecord,
        _attenuation: &mut Vec3,
        _scattered: &mut Ray3,
    ) -> bool {
        false
    }

    fn emitted(&self, u: f64, v: f64, point: Vec3) -> Vec3 {
        self.emit.value(u, v, point)
    }
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }

    pub fn color(color: Vec3) -> DiffuseLight {
        DiffuseLight::new(Arc::new(SolidTexture::new(color)))
    }
}
