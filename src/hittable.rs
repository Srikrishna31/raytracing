use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub(crate) trait Hittable {
    fn hit(self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn set_face_normal(mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = {
            let n = if self.front_face {
                outward_normal.clone()
            } else {
                -outward_normal
            };
            n.clone()
        }
    }
}
