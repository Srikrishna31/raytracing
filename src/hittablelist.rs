use crate::hittable::{HitRecord, Hittable, IntersectionInterval};
use crate::ray::Ray;
use std::boxed::Box;
use std::vec::Vec;

pub struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    hit_rec: HitRecord,
}

impl Hittable for HittableList {
    fn hit(&mut self, r: &Ray, t: &IntersectionInterval) -> Option<HitRecord> {
        // self.objects.iter().fold(None, |acc, val| {
        //     let h = val.hit(r, t);
        //     if h.is_some() {
        //         h
        //     } else {
        //         acc
        //     }
        // })

        let mut hit_anything = false;
        let mut closest_so_far = t.clone();

        for obj in self.objects.iter_mut() {
            if let Some(rec) = obj.hit(r, &closest_so_far) {
                hit_anything = true;
                closest_so_far.t = rec.t;
                self.hit_rec = rec;
            }
        }

        if hit_anything {
            Some(self.hit_rec.clone())
        } else {
            None
        }
    }
}

impl HittableList {
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }
}
