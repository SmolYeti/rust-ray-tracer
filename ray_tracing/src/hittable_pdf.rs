use crate::hittable::Hittable;
use crate::pdf::PDF;
use nurbs::vector_3::Vec3;
use std::sync::Arc;

pub struct HittablePDF {
    objects: Arc<dyn Hittable + Sync + Send>,
    origin: Vec3,
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

impl HittablePDF {
    pub fn new(objects: Arc<dyn Hittable + Sync + Send>, origin: Vec3) -> HittablePDF {
        HittablePDF { objects, origin }
    }
}
