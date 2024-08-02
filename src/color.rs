use crate::interval::Interval;
use crate::vector_3::Vec3;

pub fn vec_to_val(color: &Vec3, samples: i32) -> u32 {
    vals_to_val(color.x, color.y, color.z, samples)
}

pub fn vals_to_val(x: f64, y: f64, z: f64, samples: i32) -> u32 {
    let intensity = Interval::new(0.0, 0.999);
    let scale = 1.0 / samples as f64;
    let to_8bit = 255.999;

    // Divde color by the number of samples
    let red = x * scale;
    let green = y * scale;
    let blue = z * scale;

    // Apply the linear to gamma transform
    let red = linear_to_gamma(red);
    let green = linear_to_gamma(green);
    let blue = linear_to_gamma(blue);

    // Clamp the colors
    let red = (intensity.clamp(red) * to_8bit) as u8;
    let green = (intensity.clamp(green) * to_8bit) as u8;
    let blue = (intensity.clamp(blue) * to_8bit) as u8;

    // Return a u32
    0xFF000000 | blue as u32 | ((green as u32) << 8) | ((red as u32) << 16)
}

fn linear_to_gamma(x: f64) -> f64 {
    x.sqrt()
}
