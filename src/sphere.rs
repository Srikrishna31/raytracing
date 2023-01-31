use crate::hittable::{HitRecord, Hittable, IntersectionInterval};
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
    hit_rec: HitRecord,
}

impl Hittable for Sphere {
    fn hit(&mut self, r: &Ray, t: &IntersectionInterval) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
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
        if root < t.t_min || root > t.t_max {
            root = (-half_b + sqrtd) / a;
            if root < t.t_min || root > t.t_max {
                return None;
            }
        }

        let mut hit_rec = HitRecord {
            t: root,
            p: r.at(root),
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
        };
        let outward_normal = (hit_rec.p - self.center) / self.radius;
        hit_rec.set_face_normal(r, &outward_normal);

        Some(hit_rec)
    }
}
