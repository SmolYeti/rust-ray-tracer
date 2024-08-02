use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray3;
use crate::solid_texture::SolidTexture;
use crate::texture::Texture;
use crate::vector_3::Vec3;
use std::sync::Arc;

pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        ray_in: &Ray3,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray3,
    ) -> bool {
        let mut scatter_direction = hit_record.normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = hit_record.normal;
        }

        scattered.set_origin(hit_record.point);
        scattered.set_direction(scatter_direction);
        scattered.set_time(ray_in.time());
        attenuation.set_vec(
            self.albedo
                .value(hit_record.u, hit_record.v, hit_record.point),
        );
        true
    }
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo }
    }

    pub fn from_color(color: Vec3) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidTexture::new(color)),
        }
    }
}
