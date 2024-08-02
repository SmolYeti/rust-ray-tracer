use crate::hittable::HitRecord;
use crate::ray::Ray3;
use crate::vector_3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        ray_in: &Ray3,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray3,
    ) -> bool;

    fn emitted(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
