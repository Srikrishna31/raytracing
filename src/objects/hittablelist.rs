use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use std::rc::Rc;
use std::vec::Vec;

/// This represents the world/scene which is composed of hittables (objects). This can be used to
/// arbitrarily position a list of objects in the scene and render them.
#[derive(Clone)]
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
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
