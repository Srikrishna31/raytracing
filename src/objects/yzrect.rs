use super::{HitRecord, Hittable, AABB};
use crate::materials::Material;
use crate::{Point, Ray, Vec3};
use std::rc::Rc;

pub struct YZRect {
    y0: f64,
    y1: f64,
    z0: f64,
    z1: f64,
    k: f64,
    material: Rc<dyn Material>,
}

impl Hittable for YZRect {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().x()) / r.direction().x();
        if t < t_min || t > t_max {
            return None;
        }
        let y = r.origin().y() + t * r.direction().y();
        let z = r.origin().z() + t * r.direction().z();
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return None;
        }

        let mut rec = HitRecord {
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            t,
            mat: self.material.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: false,
        };

        let outward_normal = Vec3::new(1.0, 0.0, 0.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the X dimension a small amount
        Some(AABB::new(
            Point::new(self.k - 0.0001, self.y0, self.z0),
            Point::new(self.k + 0.0001, self.y1, self.z1),
        ))
    }
}
