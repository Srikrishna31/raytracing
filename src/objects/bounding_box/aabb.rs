use crate::{Point, Ray};

struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn min(&self) -> Point { self.minimum }

    pub fn max(&self) -> Point { self.maximum }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        false
    }
}