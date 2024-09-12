use crate::vector_3::Vec3;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
