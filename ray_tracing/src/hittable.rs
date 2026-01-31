use crate::aabb::AABB;
use crate::interval::Interval;
use crate::lambertian::Lambertian;
use crate::material::Material;
use crate::ray::Ray3;
use nurbs::vector_3::Vec3;
use std::sync::Arc;

pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material + Sync + Send>,
    pub time: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> AABB;

    fn pdf_value(&self, _origin: &Vec3, _direction: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, _origin: &Vec3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            point: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Arc::new(Lambertian::from_color(Vec3::new(1.0, 0.0, 1.0))),
            time: f64::MAX,
            u: 0.0,
            v: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray3, outward_normal: Vec3) {
        self.front_face = r.direction().dot(&outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }

    pub fn from(&mut self, rhs: &HitRecord) {
        self.point = rhs.point;
        self.normal = rhs.normal;
        self.mat = rhs.mat.clone();
        self.time = rhs.time;
        self.u = rhs.u;
        self.v = rhs.v;
        self.front_face = rhs.front_face;
    }
}
