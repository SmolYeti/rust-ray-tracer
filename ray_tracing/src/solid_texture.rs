use crate::texture::Texture;
use nurbs::vector_3::Vec3;

pub struct SolidTexture {
    color: Vec3,
}

impl Texture for SolidTexture {
    fn value(&self, _u: f64, _v: f64, _point: Vec3) -> Vec3 {
        self.color
    }
}

impl SolidTexture {
    pub fn new(color: Vec3) -> SolidTexture {
        SolidTexture { color }
    }
}
