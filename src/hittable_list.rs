use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray3;
use std::rc::Rc;

pub struct HittableList {
    pub objects: Vec<Rc<dyn Hittable>>,
    bbox: AABB,
}

unsafe impl Sync for HittableList {}
unsafe impl Send for HittableList {}

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

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        let temp_bbox = object.bounding_box();
        self.bbox = AABB::from_aabbs(&self.bbox, &temp_bbox);
        self.objects.push(object);
    }
}
