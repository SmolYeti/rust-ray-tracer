use crate::perlin::Perlin;
use crate::texture::Texture;
use nurbs::vector_3::Vec3;

pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, point: Vec3) -> Vec3 {
        let scaled_point = point * self.scale;
        Vec3::new(1.0, 1.0, 1.0)
            * 0.5
            * (1.0 + (scaled_point.z + 10.0 * self.noise.turbulence(scaled_point, 4)).sin())
    }
}

impl NoiseTexture {
    pub fn new(scale: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Perlin::new(),
            scale,
        }
    }
}
