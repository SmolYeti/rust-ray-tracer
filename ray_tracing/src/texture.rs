use nurbs::vector_3::Vec3;

pub trait Texture {
    fn value(&self, u: f64, v: f64, point: Vec3) -> Vec3;
}
