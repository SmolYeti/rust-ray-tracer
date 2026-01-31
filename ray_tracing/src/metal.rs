use crate::hittable::HitRecord;
use crate::material::{Material, ScatterPDF, ScatterRecord};
use crate::ray::Ray3;
use nurbs::vector_3::Vec3;

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray3, hit_rec: &HitRecord, scatter_rec: &mut ScatterRecord) -> bool {
        let reflected = ray_in.direction().reflect(&hit_rec.normal).unit_vector()
            + (self.fuzz * Vec3::random_in_unit_sphere());

        scatter_rec.attenuation = self.albedo;
        scatter_rec.pdf = ScatterPDF::Skip(Ray3::new(hit_rec.point, reflected, ray_in.time()));
        true
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Metal {
        Metal { albedo, fuzz }
    }
}
