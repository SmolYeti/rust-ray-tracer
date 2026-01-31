use rand;

pub fn degree_to_radians(degree: f64) -> f64 {
    degree * std::f64::consts::PI / 180.0
}

pub fn random_f64_range(min: f64, max: f64) -> f64 {
    ((max - min) * rand::random::<f64>()) + min
}

pub fn random_u32_range(min: u32, max: u32) -> u32 {
    (random_f64_range(min as f64, (max + 1) as f64)) as u32
}
