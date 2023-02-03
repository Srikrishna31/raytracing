use crate::hittable::{HitRecord, Hittable};
use crate::lambertian::LambertianMaterial;
use crate::ray::Ray;
use crate::Color;
use std::rc::Rc;
use std::vec::Vec;

/// This represents the world/scene which is composed of hittables (objects). This can be used to
/// arbitrarily position a list of objects in the scene and render them.
pub struct HittableList {
    objects: Vec<Rc<dyn Hittable>>,
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // self.objects.iter().fold(None, |acc, val| {
        //     let h = val.hit(r, t);
        //     if h.is_some() {
        //         h
        //     } else {
        //         acc
        //     }
        // })

        let mut temp_rec: HitRecord =
            HitRecord::new(Rc::new(LambertianMaterial::new(Color::new(0.0, 0.0, 0.0))));
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for obj in self.objects.iter() {
            if let Some(rec) = obj.hit(r, t_min, closest_so_far) {
                hit_anything = true;
                closest_so_far = rec.t;
                temp_rec = rec;
            }
        }

        if hit_anything {
            Some(temp_rec)
        } else {
            None
        }
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
