use crate::pdf::PDF;
use nurbs::vector_3::Vec3;
use std::f64::consts::FRAC_1_PI;

pub struct SpherePDF {}

impl PDF for SpherePDF {
    fn value(&self, _direction: &Vec3) -> f64 {
        0.25 * FRAC_1_PI
    }

    fn generate(&self) -> Vec3 {
        Vec3::random_unit_vector()
    }
}
