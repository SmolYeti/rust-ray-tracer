use crate::vector_3::Vec3;

use rand::{seq::SliceRandom, thread_rng};

const PERLIN_POINT_COUNT: i32 = 256;

pub struct Perlin {
    rand_vec3: Vec<Vec3>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    fn generate_perm() -> Vec<i32> {
        let mut vec: Vec<i32> = (0..PERLIN_POINT_COUNT).collect();
        let mut rng = thread_rng();
        vec.shuffle(&mut rng);
        vec
    }

    pub fn new() -> Perlin {
        let mut rand_vec3 = Vec::<Vec3>::with_capacity(PERLIN_POINT_COUNT as usize);
        for _ in 0..PERLIN_POINT_COUNT {
            rand_vec3.push(Vec3::random_range(-1.0, 1.0).unit_vector());
        }

        let perm_x = Self::generate_perm();
        let perm_y = Self::generate_perm();
        let perm_z = Self::generate_perm();

        Perlin {
            rand_vec3,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, point: Vec3) -> f64 {
        let u = point.x - point.x.floor();
        let v = point.y - point.y.floor();
        let w = point.z - point.z.floor();

        let uu = u * u * (3.0 - (2.0 * u));
        let vv = v * v * (3.0 - (2.0 * v));
        let ww = w * w * (3.0 - (2.0 * w));

        let i = point.x.floor() as i32;
        let j = point.y.floor() as i32;
        let k = point.z.floor() as i32;

        let mut accumulate = 0.0;
        for di in 0..2 {
            let di_f = di as f64;
            let du = di_f * uu + (1.0 - di_f) * (1.0 - uu);

            for dj in 0..2 {
                let dj_f = dj as f64;
                let dv = dj_f * vv + (1.0 - dj_f) * (1.0 - vv);

                for dk in 0..2 {
                    let dk_f = dk as f64;
                    let dw = dk_f * ww + (1.0 - dk_f) * (1.0 - ww);

                    let c = self.rand_vec3[(self.perm_x[((i + di) & 255) as usize]
                        ^ self.perm_y[((j + dj) & 255) as usize]
                        ^ self.perm_z[((k + dk) & 255) as usize])
                        as usize];

                    let weight = Vec3::new(u - di_f, v - dj_f, w - dk_f);

                    accumulate += du * dv * dw * c.dot(&weight);
                }
            }
        }

        accumulate
    }

    pub fn turbulence(&self, point: Vec3, depth: i32) -> f64 {
        let mut accumulate = 0.0;
        let mut temp_point = point;
        let mut weight = 1.0;

        for _ in 0..depth {
            accumulate += weight * self.noise(temp_point.clone());
            weight *= 0.5;
            temp_point = temp_point * 2.0;
        }

        accumulate.abs()
    }
}
