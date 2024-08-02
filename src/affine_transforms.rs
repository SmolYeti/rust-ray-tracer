use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray3;
use crate::rtweekend;
use crate::vector_3::Vec3;
use std::f64::INFINITY;
use std::rc::Rc;

pub struct Translate {
    object: Rc<dyn Hittable>,
    bbox: AABB,
    offset: Vec3,
}

pub struct RotateY {
    object: Rc<dyn Hittable>,
    bbox: AABB,
    sin_theta: f64,
    cos_theta: f64,
}

impl Hittable for Translate {
    fn hit(&self, ray_in: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let offset_ray = Ray3::new(
            ray_in.origin() - self.offset,
            ray_in.direction(),
            ray_in.time(),
        );

        if self.object.hit(&offset_ray, time, hit_record) {
            hit_record.point += self.offset;
            true
        } else {
            false
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.copy()
    }
}

impl Hittable for RotateY {
    fn hit(&self, ray_in: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let mut origin = ray_in.origin();
        let mut direction = ray_in.direction();

        origin.x = self.cos_theta * ray_in.origin().x - self.sin_theta * ray_in.origin().z;
        origin.z = self.sin_theta * ray_in.origin().x + self.cos_theta * ray_in.origin().z;

        direction.x = self.cos_theta * ray_in.direction().x - self.sin_theta * ray_in.direction().z;
        direction.z = self.sin_theta * ray_in.direction().x + self.cos_theta * ray_in.direction().z;

        let rotated_ray = Ray3::new(origin, direction, ray_in.time());

        if self.object.hit(&rotated_ray, time, hit_record) {
            let mut point = hit_record.point.clone();
            point.x = self.cos_theta * hit_record.point.x - self.sin_theta * hit_record.point.z;
            point.z = self.sin_theta * hit_record.point.x + self.cos_theta * hit_record.point.z;

            let mut normal = hit_record.normal.clone();
            normal.x = self.cos_theta * hit_record.normal.x - self.sin_theta * hit_record.normal.z;
            normal.z = self.sin_theta * hit_record.normal.x + self.cos_theta * hit_record.normal.z;

            hit_record.point = point;
            hit_record.normal = normal;

            true
        } else {
            false
        }
    }

    fn bounding_box(&self) -> AABB {
        self.bbox.copy()
    }
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Translate {
        let bbox = object.bounding_box() + offset;
        Translate {
            object,
            bbox,
            offset,
        }
    }
}

impl RotateY {
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> RotateY {
        let radians = rtweekend::degree_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Vec3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let i_f = i as f64;
                    let j_f = j as f64;
                    let k_f = k as f64;

                    let x = i_f * bbox.axis(0).max() + (1.0 - i_f) * bbox.axis(0).min();
                    let y = j_f * bbox.axis(1).max() + (1.0 - j_f) * bbox.axis(1).min();
                    let z = k_f * bbox.axis(2).max() + (1.0 - k_f) * bbox.axis(2).min();

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for index in 0..3 {
                        min.set(index, min.at(index).min(tester.at(index)));
                        max.set(index, max.at(index).max(tester.at(index)));
                    }
                }
            }
        }

        let bbox = AABB::from_vec3s(min, max);

        RotateY {
            object,
            bbox,
            sin_theta,
            cos_theta,
        }
    }
}
