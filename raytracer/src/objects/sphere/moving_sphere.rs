use super::common;
use crate::materials::Material;
use crate::objects::{HitRecord, Hittable, AABB};
use crate::{Point, Ray, Vec3};
use std::sync::Arc;

pub struct MovingSphere {
    center0: Point,
    center1: Point,
    radius: f64,
    material: Arc<dyn Material>,
    time0: f64,
    time1: f64,
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        common::hit(r, t_min, t_max, &|pt| pt - self.center(r.time()), &|t|self.center(t), self.radius, self.material.clone())
    }

    /// For `MovingSphere`, we can take the box of the sphere at t<sub>0</sub>, and the box of the
    /// sphere at t<sub>1</sub>, and compute the box of those two boxes
    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        let radius_dir = Vec3::new(self.radius, self.radius, self.radius);

        let box1 = AABB::new(
            self.center(time0) - radius_dir,
            self.center(time0) + radius_dir,
        );

        let box2 = AABB::new(
            self.center(time1) - radius_dir,
            self.center(time1) + radius_dir,
        );

        Some(AABB::surrounding_box(&box1, &box2))
    }
}

impl MovingSphere {
    fn center(&self, time: f64) -> Point {
        self.center0
            + ((time - self.time0) / (self.time1 - self.time0)) * (self.center1 - self.center0)
    }

    pub fn new(
        center0: Point,
        center1: Point,
        radius: f64,
        material: Arc<dyn Material>,
        time0: f64,
        time1: f64,
    ) -> Result<MovingSphere, String> {
        if time0 < 0.0 || time1 < 0.0 {
            let err = format!("Time cannot be negative, {time0}, {time1}");
            return Err(err);
        }

        if time1 < time0 {
            let err = format!("End time cannot be less than start time: {time0}, {time1}");
            return Err(err);
        }

        Ok(MovingSphere {
            center0,
            center1,
            radius,
            material,
            time0,
            time1,
        })
    }
}
