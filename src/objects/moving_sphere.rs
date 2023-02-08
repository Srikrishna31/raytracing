use crate::materials::Material;
use crate::objects::{HitRecord, Hittable};
use crate::{Point, Ray, Vec3};
use std::rc::Rc;

pub struct MovingSphere {
    center0: Point,
    center1: Point,
    radius: f64,
    material: Rc<dyn Material>,
    time0: f64,
    time1: f64,
}

impl Hittable for MovingSphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center(r.time());
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let mut hit_rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: self.material.clone(),
        };
        let outward_normal = (hit_rec.p - self.center(r.time())) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);

        Some(hit_rec)
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
        material: Rc<dyn Material>,
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
