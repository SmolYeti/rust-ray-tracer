use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::isotropic::Isotropic;
use crate::material::Material;
use crate::ray::Ray3;
//use crate::texture::Texture;
use crate::vector_3::Vec3;
use std::f64::INFINITY;
use std::rc::Rc;
use std::sync::Arc;

pub struct ConstantMedium {
    boundary: Rc<dyn Hittable>,
    phase_func: Arc<dyn Material>,
    neg_inv_density: f64,
}

impl Hittable for ConstantMedium {
    fn hit(&self, ray_in: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let mut rec_1 = HitRecord::new();
        let mut rec_2 = HitRecord::new();

        let has_hit = if self.boundary.hit(ray_in, Interval::univeral(), &mut rec_1)
            && self.boundary.hit(
                ray_in,
                Interval::new(rec_1.time + 0.0001, INFINITY),
                &mut rec_2,
            ) {
            if rec_1.time < time.min() {
                rec_1.time = time.min();
            }
            if rec_2.time > time.max() {
                rec_2.time = time.max();
            }

            if rec_1.time >= rec_2.time {
                false
            } else {
                if rec_1.time < 0.0 {
                    rec_1.time = 0.0;
                }

                let ray_length = ray_in.direction().length();
                let distance_inside_boundary = (rec_2.time - rec_1.time) * ray_length;
                let hit_dist = self.neg_inv_density * rand::random::<f64>().log10();

                if hit_dist > distance_inside_boundary {
                    false
                } else {
                    hit_record.time = rec_1.time + hit_dist / ray_length;
                    hit_record.point = ray_in.at(hit_record.time);
                    hit_record.normal = Vec3::new(1.0, 0.0, 0.0);
                    hit_record.front_face = true;
                    hit_record.mat = Arc::clone(&self.phase_func);
                    true
                }
            }
        } else {
            false
        };

        has_hit
    }

    fn bounding_box(&self) -> AABB {
        self.boundary.bounding_box()
    }
}

impl ConstantMedium {
    // pub fn new(
    //     boundary: Rc<dyn Hittable>,
    //     density: f64,
    //     phase_func: Arc<dyn Texture>,
    // ) -> ConstantMedium {
    //     ConstantMedium {
    //         boundary,
    //         phase_func: Arc::new(Isotropic::new(phase_func)),
    //         neg_inv_density: -1.0 / density,
    //     }
    // }

    pub fn color(boundary: Rc<dyn Hittable>, density: f64, color: Vec3) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_func: Arc::new(Isotropic::color(color)),
            neg_inv_density: -1.0 / density,
        }
    }
}
