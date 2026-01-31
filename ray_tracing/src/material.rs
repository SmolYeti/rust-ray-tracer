use crate::hittable::HitRecord;
use crate::pdf::PDF;
use crate::ray::Ray3;
use nurbs::vector_3::Vec3;

pub enum ScatterPDF {
    PDF(Box<dyn PDF>),
    Skip(Ray3),
}
pub struct ScatterRecord {
    pub attenuation: Vec3,
    pub pdf: ScatterPDF,
}

pub trait Material {
    fn scatter(
        &self,
        _ray_in: &Ray3,
        _hit_rec: &HitRecord,
        _scatter_rec: &mut ScatterRecord,
    ) -> bool {
        false
    }

    fn scattering_pdf(&self, _ray_in: &Ray3, _hit_record: &HitRecord, _scattered: &Ray3) -> f64 {
        0.0
    }

    fn emitted(
        &self,
        _ray_in: &Ray3,
        _hit_record: &HitRecord,
        _u: f64,
        _v: f64,
        _point: Vec3,
    ) -> Vec3 {
        Vec3::new(0.0, 0.0, 0.0)
    }
}

impl ScatterRecord {
    pub fn new() -> ScatterRecord {
        let attenuation = Vec3::empty();
        let pdf = ScatterPDF::Skip(Ray3::empty());
        ScatterRecord { attenuation, pdf }
    }
}
