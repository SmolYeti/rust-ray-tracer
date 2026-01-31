use crate::pdf::PDF;
use nurbs::vector_3::Vec3;

pub struct MixturePDF {
    pdf_0: Box<dyn PDF>,
    pdf_1: Box<dyn PDF>,
}

impl PDF for MixturePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.pdf_0.value(direction) + 0.5 * self.pdf_1.value(direction)
    }

    fn generate(&self) -> Vec3 {
        if rand::random::<f64>() < 0.5 {
            self.pdf_0.generate()
        } else {
            self.pdf_1.generate()
        }
    }
}

impl MixturePDF {
    pub fn new(pdf_0: Box<dyn PDF>, pdf_1: Box<dyn PDF>) -> MixturePDF {
        MixturePDF { pdf_0, pdf_1 }
    }
}
