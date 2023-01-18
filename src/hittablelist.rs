use std::boxed::Box;
use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use std::vec::Vec;

struct HittableList {
    objects: Vec<Box<dyn Hittable>>,
    hit_rec: HitRecord,
}

impl Hittable for HittableList {
    fn hit(mut self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {

        self.objects.iter().fold(None, |acc, val| {
            let h = val.hit(r, t_min, t_max);
            if h.is_some() {
                h
            } else {
                acc
            }
        })

        // let temp_rec: HitRecord;
        // let mut hit_anything = false;
        // let mut closest_so_far = t_max;

        // for obj in self.objects {
        //     if obj.hit(r, t_min, t_max, &temp_rec) {
        //         hit_anything = true;
        //         closest_so_far = temp_rec.t;
        //         self.rec = temp_rec;
        //     }
        // }
        //
        // hit_anything
    }
}