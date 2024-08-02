use std::cmp::Ordering;
use std::rc::Rc;

use crate::aabb::AABB;
use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::ray::Ray3;
use crate::rtweekend::random_u32_range;

pub struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    fn hit(&self, ray_in: &Ray3, time: Interval, hit_record: &mut HitRecord) -> bool {
        let hit_anything = if self.bbox.hit(ray_in, time.copy()) {
            let hit_left = self.left.hit(ray_in, time.copy(), hit_record);
            let hit_right = self.right.hit(
                ray_in,
                Interval::new(
                    time.min(),
                    if hit_left {
                        hit_record.time
                    } else {
                        time.max()
                    },
                ),
                hit_record,
            );
            hit_left || hit_right
        } else {
            false
        };
        hit_anything
    }

    fn bounding_box(&self) -> AABB {
        AABB::copy(&self.bbox)
    }
}

impl BVHNode {
    pub fn from_vec(objects: &Vec<Rc<dyn Hittable>>, start: usize, end: usize) -> BVHNode {
        let axis = random_u32_range(0, 2);
        let comparator = if axis == 0 {
            BVHNode::box_compare_x
        } else if axis == 1 {
            BVHNode::box_compare_y
        } else {
            BVHNode::box_compare_z
        };

        let span = end - start;
        let mut left = Rc::clone(&objects[start]);
        let mut right = Rc::clone(&objects[start]);
        if span == 2 {
            if comparator(&objects[start], &objects[start + 1]) {
                right = Rc::clone(&objects[start + 1]);
            } else {
                left = Rc::clone(&objects[start + 1]);
            }
        } else if span != 1 {
            let mut slice = objects[start..end].to_vec();
            slice.sort_by(|a, b| {
                if comparator(a, b) {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });

            let start = 0;
            let end = slice.len();
            let mid = start + (span / 2);

            left = Rc::new(BVHNode::from_vec(&slice, start, mid));
            right = Rc::new(BVHNode::from_vec(&slice, mid, end));
        }
        let bbox = AABB::from_aabbs(&left.bounding_box().copy(), &right.bounding_box().copy());
        BVHNode { left, right, bbox }
    }

    pub fn from_list(list: &HittableList) -> BVHNode {
        BVHNode::from_vec(&list.objects, 0, list.objects.len())
    }

    fn box_compare(left: &Rc<dyn Hittable>, right: &Rc<dyn Hittable>, axis: u32) -> bool {
        left.bounding_box().axis(axis).min() < right.bounding_box().axis(axis).min()
    }

    fn box_compare_x(left: &Rc<dyn Hittable>, right: &Rc<dyn Hittable>) -> bool {
        BVHNode::box_compare(left, right, 0)
    }

    fn box_compare_y(left: &Rc<dyn Hittable>, right: &Rc<dyn Hittable>) -> bool {
        BVHNode::box_compare(left, right, 1)
    }

    fn box_compare_z(left: &Rc<dyn Hittable>, right: &Rc<dyn Hittable>) -> bool {
        BVHNode::box_compare(left, right, 2)
    }
}
