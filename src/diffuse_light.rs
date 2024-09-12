use crate::hittable::HitRecord;
use crate::material::{Material, ScatterRecord};
use crate::ray::Ray3;
use crate::solid_texture::SolidTexture;
use crate::texture::Texture;
use crate::vector_3::Vec3;
use std::sync::Arc;

pub struct DiffuseLight {
    emit: Arc<dyn Texture + Sync + Send>,
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _ray_in: &Ray3,
        _hit_rec: &HitRecord,
        _scatter_rec: &mut ScatterRecord,
    ) -> bool {
        false
    }

    fn emitted(&self, _ray_in: &Ray3, hit_record: &HitRecord, u: f64, v: f64, point: Vec3) -> Vec3 {
        if !hit_record.front_face {
            Vec3::new(0.0, 0.0, 1.0)
        } else {
            self.emit.value(u, v, point)
        }
    }
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture + Sync + Send>) -> DiffuseLight {
        DiffuseLight { emit }
    }

    pub fn color(color: Vec3) -> DiffuseLight {
        DiffuseLight::new(Arc::new(SolidTexture::new(color)))
    }
}
