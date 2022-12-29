use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::Point;

struct Sphere {
    center: Point,
    radius: f64,
    hit_rec: HitRecord,
}

impl Hittable for Sphere {
    fn hit(mut self, r: &Ray, t_min: f64, t_max: f64, rec: &HitRecord) -> bool {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return false;
            }
        }

        self.hit_rec.t = root;
        self.hit_rec.p = r.at(rec.t);
        self.hit_rec.normal = (rec.p - self.center) / self.radius;

        true
    }
}