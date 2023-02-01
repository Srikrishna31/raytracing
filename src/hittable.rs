use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3 { e: [0.0, 0.0, 0.0] },
            normal: Vec3 { e: [0.0, 0.0, 0.0] },
            t: 0.0,
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            } else {
                -outward_normal
            }
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
