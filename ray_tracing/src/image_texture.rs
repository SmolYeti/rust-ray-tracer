use crate::interval::Interval;
use crate::texture::Texture;
use nurbs::vector_3::Vec3;
use image::io;
use image::RgbImage;

pub struct ImageTexture {
    image: RgbImage,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _point: Vec3) -> Vec3 {
        let mut return_color = Vec3::new(1.0, 0.0, 1.0);
        if self.image.height() > 0 {
            let u = Interval::new(0.0, 1.0).clamp(u);
            let v = 1.0 - Interval::new(0.0, 1.0).clamp(v);

            let i = (u * (self.image.width() - 1) as f64) as u32;
            let j = (v * (self.image.height() - 1) as f64) as u32;

            let pixel = self.image.get_pixel(i, j);

            let color_scale = 1.0 / 255.0;
            return_color = Vec3::new(
                color_scale * pixel.0[0] as f64,
                color_scale * pixel.0[1] as f64,
                color_scale * pixel.0[2] as f64,
            );
        }
        return_color
    }
}

impl ImageTexture {
    pub fn new(path: &str) -> ImageTexture {
        let image = io::Reader::open(path)
            .unwrap()
            .decode()
            .unwrap()
            .into_rgb8();
        ImageTexture { image }
    }
}
