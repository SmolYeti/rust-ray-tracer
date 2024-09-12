use crate::orthonormal_basis::OrthonormalBasis;
use crate::pdf::PDF;
use crate::vector_3::Vec3;
use std::f64::consts::FRAC_1_PI;

pub struct CosinePDF {
    uvw: OrthonormalBasis,
}

impl PDF for CosinePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cos_theta = direction.unit_vector().dot(&self.uvw.w());
        f64::max(0.0, cos_theta * FRAC_1_PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(Vec3::random_cosine_direction())
    }
}

impl CosinePDF {
    pub fn new(normal: &Vec3) -> CosinePDF {
        CosinePDF {
            uvw: OrthonormalBasis::new(normal),
        }
    }
}
