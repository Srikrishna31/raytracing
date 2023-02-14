use super::hittablelist::HittableList;
use crate::materials::Material;
use crate::objects::{HitRecord, Hittable, XYRect, XZRect, YZRect, AABB};
use crate::{Point, Ray};
use std::rc::Rc;

pub struct Box {
    min: Point,
    max: Point,
    sides: HittableList,
}

impl Hittable for Box {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.sides.hit(r, t_min, t_max)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        Some(AABB::new(self.min, self.max))
    }
}

impl Box {
    pub fn new(min: Point, max: Point, material: Rc<dyn Material>) -> Box {
        let mut sides = HittableList::new();

        sides.add(Rc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            max.z(),
            material.clone(),
        )));
        sides.add(Rc::new(XYRect::new(
            min.x(),
            max.x(),
            min.y(),
            max.y(),
            min.z(),
            material.clone(),
        )));

        sides.add(Rc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            max.y(),
            material.clone(),
        )));
        sides.add(Rc::new(XZRect::new(
            min.x(),
            max.x(),
            min.z(),
            max.z(),
            min.y(),
            material.clone(),
        )));

        sides.add(Rc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            max.x(),
            material.clone(),
        )));
        sides.add(Rc::new(YZRect::new(
            min.y(),
            max.y(),
            min.z(),
            max.z(),
            min.x(),
            material,
        )));

        Box { min, max, sides }
    }
}
