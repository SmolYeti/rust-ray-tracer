use crate::hittable::HitRecord;
use crate::material::Material;
use crate::ray::Ray3;
use crate::vector_3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(
        &self,
        ray_in: &Ray3,
        hit_record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray3,
    ) -> bool {
        let reflected = ray_in.direction().unit_vector().reflect(&hit_record.normal)
            + (self.fuzz * Vec3::random_in_unit_sphere());
        scattered.set_origin(hit_record.point);
        scattered.set_direction(reflected);
        scattered.set_time(ray_in.time());
        attenuation.set_vec(self.albedo);
        scattered.direction().dot(&hit_record.normal) > 0.0
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}
