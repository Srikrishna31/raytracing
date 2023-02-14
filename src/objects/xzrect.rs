use super::{HitRecord, Hittable, AABB};
use crate::{materials::Material, Point, Ray, Vec3};
use std::rc::Rc;

pub struct XZRect {
    x0: f64,
    x1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

impl Hittable for XZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().y()) / r.direction().y();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let z = r.origin().z() + t * r.direction().z();
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            mat: self.material.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: false,
        };

        let outward_normal = Vec3::new(0.0, 1.0, 0.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Y dimension a small amount
        Some(AABB::new(
            Point::new(self.x0, self.k - 0.0001, self.z0),
            Point::new(self.x1, self.k + 0.0001, self.z1),
        ))
    }
}

impl XZRect {
    pub fn new(x0: f64, x1: f64, z0: f64, z1: f64, k: f64, material: Rc<dyn Material>) -> XZRect {
        XZRect {
            x0,
            x1,
            z0,
            z1,
            k,
            material,
        }
    }
}
