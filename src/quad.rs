use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray3;
use crate::vector_3::Vec3;
use std::rc::Rc;
use std::sync::Arc;

pub struct Quad {
    mat: Arc<dyn Material>,
    bbox: AABB,
    origin: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    normal: Vec3,
    plane_const: f64,
}

impl Hittable for Quad {
    fn hit(&self, ray_in: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let denom = self.normal.dot(&ray_in.direction());

        let mut quad_hit = false;
        if denom.abs() >= 1e-8 {
            let hit_time = (self.plane_const - self.normal.dot(&ray_in.origin())) / denom;

            if time.contains(hit_time) {
                let intersection = ray_in.at(hit_time);

                let hitpt_vector = intersection - self.origin;
                let alpha = self.w.dot(&hitpt_vector.cross(&self.v));
                let beta = self.w.dot(&self.u.cross(&hitpt_vector));

                if Quad::interior(alpha, beta, hit_record) {
                    hit_record.time = hit_time;
                    hit_record.point = intersection;
                    hit_record.mat = Arc::clone(&self.mat);
                    hit_record.set_face_normal(ray_in, self.normal);

                    quad_hit = true;
                }
            }
        }
        quad_hit
    }

    fn bounding_box(&self) -> crate::aabb::AABB {
        AABB::copy(&self.bbox)
    }
}

impl Quad {
    pub fn new(origin: Vec3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
        let bbox = AABB::from_vec3s(origin, origin + u + v).pad();
        let n = u.cross(&v);
        let normal = n.unit_vector();
        let plane_const = normal.dot(&origin);
        let w = n / n.dot(&n);
        Quad {
            mat,
            bbox,
            origin,
            u,
            v,
            w,
            normal,
            plane_const,
        }
    }

    fn interior(a: f64, b: f64, hit_record: &mut HitRecord) -> bool {
        if a < 0.0 || 1.0 < a || b < 0.0 || 1.0 < b {
            false
        } else {
            hit_record.u = a;
            hit_record.v = b;
            true
        }
    }
}

pub fn quad_box(point_a: Vec3, point_b: Vec3, mat: Arc<dyn Material>) -> Rc<HittableList> {
    let bbox = AABB::from_vec3s(point_a, point_b);

    let min = Vec3::new(bbox.axis(0).min(), bbox.axis(1).min(), bbox.axis(2).min());
    let max = Vec3::new(bbox.axis(0).max(), bbox.axis(1).max(), bbox.axis(2).max());

    let dx = Vec3::new(bbox.axis(0).size(), 0.0, 0.0);
    let dy = Vec3::new(0.0, bbox.axis(1).size(), 0.0);
    let dz = Vec3::new(0.0, 0.0, bbox.axis(2).size());

    let mut quads = HittableList::new();

    // front
    quads.add(Rc::new(Quad::new(
        Vec3::new(min.x, min.y, max.z),
        dx,
        dy,
        Arc::clone(&mat),
    )));
    // right
    quads.add(Rc::new(Quad::new(
        Vec3::new(max.x, min.y, max.z),
        -dz,
        dy,
        Arc::clone(&mat),
    )));
    // back
    quads.add(Rc::new(Quad::new(
        Vec3::new(max.x, min.y, min.z),
        -dx,
        dy,
        Arc::clone(&mat),
    )));
    // left
    quads.add(Rc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dz,
        dy,
        Arc::clone(&mat),
    )));
    // top
    quads.add(Rc::new(Quad::new(
        Vec3::new(min.x, max.y, max.z),
        dx,
        -dz,
        Arc::clone(&mat),
    )));
    // bottom
    quads.add(Rc::new(Quad::new(
        Vec3::new(min.x, min.y, min.z),
        dx,
        dz,
        Arc::clone(&mat),
    )));

    Rc::new(quads)
}
