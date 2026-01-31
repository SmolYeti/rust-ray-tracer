use crate::aabb::AABB;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray3;
use nurbs::{utility::random_u32_range, vector_3::Vec3};
use std::sync::Arc;

pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync + Send>>,
    bbox: AABB,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let mut temp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = time.max();

        for object in &self.objects {
            if object.hit(
                r,
                Interval::new(time.min(), closest_so_far),
                &mut temp_record,
            ) {
                hit_anything = true;
                closest_so_far = temp_record.time;
                hit_record.from(&temp_record);
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        AABB::copy(&self.bbox)
    }

    fn pdf_value(&self, origin: &Vec3, direction: &Vec3) -> f64 {
        let weight = 1.0 / (self.objects.len() as f64);
        let mut sum = 0.0;
        for obj in &self.objects {
            sum += weight * obj.pdf_value(origin, direction);
        }
        sum
    }

    fn random(&self, origin: &Vec3) -> Vec3 {
        let size = self.objects.len() as u32;
        if size == 0 {
            Vec3::new(1.0, 0.0, 0.0)
        } else {
            self.objects[random_u32_range(0, size - 1) as usize].random(origin)
        }
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::empty(),
        }
    }

    /*pub fn clear(&mut self) {
        self.objects.clear();
    }*/

    pub fn add(&mut self, object: Arc<dyn Hittable + Sync + Send>) {
        let temp_bbox = object.bounding_box();
        self.bbox = AABB::from_aabbs(&self.bbox, &temp_bbox);
        self.objects.push(object);
    }
}
