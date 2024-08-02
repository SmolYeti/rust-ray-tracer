use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray3;
use crate::vector_3::Vec3;

pub struct Sphere {
    center_start: Vec3,
    radius: f64,
    mat: Arc<dyn Material>,
    b_moving: bool,
    center_move: Vec3,
    bbox: AABB,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let center = if self.b_moving {
            self.center(r.time())
        } else {
            self.center_start
        };
        let oc = r.origin() - center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - (self.radius * self.radius);

        let discriminant = (half_b * half_b) - (a * c);
        if discriminant < 0.0 {
            false
        } else {
            let d_sqrt = discriminant.sqrt();

            // Find the nearest root that lies in the acceptable range
            let mut root = (-half_b - d_sqrt) / a;
            if !time.surrounds(root) {
                root = (-half_b + d_sqrt) / a;
            }
            if !time.surrounds(root) {
                false
            } else {
                // Calculate the hit
                hit_record.time = root;
                hit_record.point = r.at(hit_record.time);
                let outward_normal = (hit_record.point - center) / self.radius;
                hit_record.set_face_normal(r, outward_normal);
                hit_record.mat = Arc::clone(&self.mat);
                Sphere::sphere_uv(outward_normal, &mut hit_record.u, &mut hit_record.v);
                true
            }
        }
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        AABB::copy(&self.bbox)
    }
}

impl Sphere {
    pub fn new(center_start: Vec3, radius: f64, mat: Arc<dyn Material>) -> Sphere {
        let radius_vec = Vec3::new(radius, radius, radius);
        Sphere {
            center_start,
            radius,
            mat,
            b_moving: false,
            center_move: Vec3::empty(),
            bbox: AABB::from_vec3s(center_start - radius_vec, center_start + radius_vec),
        }
    }

    pub fn new_moving(
        center_start: Vec3,
        radius: f64,
        mat: Arc<dyn Material>,
        center_end: Vec3,
    ) -> Sphere {
        let radius_vec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::from_vec3s(center_start - radius_vec, center_start + radius_vec);
        let bbox2 = AABB::from_vec3s(center_end - radius_vec, center_end + radius_vec);
        Sphere {
            center_start,
            radius,
            mat,
            b_moving: true,
            center_move: center_end - center_start,
            bbox: AABB::from_aabbs(&bbox1, &bbox2),
        }
    }

    pub fn center(&self, time: f64) -> Vec3 {
        self.center_start + self.center_move * time
    }

    pub fn sphere_uv(point: Vec3, u: &mut f64, v: &mut f64) {
        let theta = (-point.y).acos();
        let phi = (-point.z).atan2(point.x) + PI;

        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}
