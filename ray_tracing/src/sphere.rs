use core::f64;
use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::Material;
use crate::orthonormal_basis::OrthonormalBasis;
use crate::ray::Ray3;
use nurbs::vector_3::Vec3;

pub struct Sphere {
    center_start: Vec3,
    radius: f64,
    mat: Arc<dyn Material + Sync + Send>,
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

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        // Only for stationary spheres
        let mut record = HitRecord::new();

        let ray = Ray3::new(*origin, *direction, 0.0);
        if self.hit(&ray, Interval::new(0.001, f64::INFINITY), &mut record) {
            let dist_sq = (self.center(0.0) - *origin).length_squared();
            let cos_theta = f64::sqrt(1.0 - ((self.radius * self.radius) / dist_sq));
            let solid_angle = 2.0 * PI * (1.0 - cos_theta);
            1.0 / solid_angle
        } else {
            0.0
        }
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let dir = self.center(0.0) - *origin;
        let dist_sq = dir.length_squared();
        let uvw = OrthonormalBasis::new(&dir);
        uvw.transform(Sphere::random_to_sphere(self.radius, dist_sq))
    }
}

impl Sphere {
    pub fn new(center_start: Vec3, radius: f64, mat: Arc<dyn Material + Sync + Send>) -> Sphere {
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
        mat: Arc<dyn Material + Sync + Send>,
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

    pub fn random_to_sphere(radius: f64, dist_sq: f64) -> Vec3 {
        let r1 = rand::random::<f64>();
        let r2 = rand::random::<f64>();
        let z = 1.0 + r2 * (f64::sqrt(1.0 - ((radius * radius) / dist_sq)) - 1.0);

        let z_sqrt = f64::sqrt(1.0 - (z * z));
        let phi = 2.0 * PI * r1;
        let x = f64::cos(phi) * z_sqrt;
        let y = f64::sin(phi) * z_sqrt;

        Vec3::new(x, y, z)
    }
}
