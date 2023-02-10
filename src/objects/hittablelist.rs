use crate::objects::{HitRecord, Hittable, AABB};
use crate::ray::Ray;
use std::rc::Rc;
use std::vec::Vec;

/// This represents the world/scene which is composed of hittables (objects). This can be used to
/// arbitrarily position a list of objects in the scene and render them.
#[derive(Clone)]
pub struct HittableList {
    pub(in crate::objects) objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;

        // TODO: Investigate a way to parallelize this function.
        self.objects
            .iter()
            .fold(None, |acc, val| match val.hit(r, t_min, closest_so_far) {
                Some(rec) => {
                    closest_so_far = rec.t;
                    Some(rec)
                }
                None => acc,
            })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        if self.objects.is_empty() {
            return None;
        }

        let mut first_box = true;

        // TODO: use fold that returns early from the collection.
        self.objects.iter().fold(None, |acc, val| {
            match (acc, val.bounding_box(time0, time1)) {
                (_, None) => None, // Even if one object doesn't have a bounding box
                (None, Some(bbox)) => {
                    if first_box {
                        first_box = false;
                        Some(bbox)
                    } else {
                        // This means there was an object whose box is not hit.
                        None
                    }
                }
                (Some(bbox1), Some(bbox2)) => Some(AABB::surrounding_box(&bbox1, &bbox2)),
            }
        })
    }
}

impl HittableList {
    pub fn new() -> HittableList {
        HittableList { objects: vec![] }
    }

    pub fn add(&mut self, object: Rc<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}
