use crate::materials::Material;
use embed_doc_image::embed_doc_image;

use crate::objects::{HitRecord, Hittable, AABB};
use crate::{Point, Ray, Vec3};
use std::rc::Rc;

/// # Creating Rectangle Objects
/// Rectangles are often convenient for modeling man-made environments. First, below is a rectangle
/// in an xy plane. Such a plane is defined by its *z* value. For example *z = **k***. An axis-aligned
/// rectangle is defined by the lines *x=x<sub>0</sub>, x=x<sub>1</sub>, y=y<sub>0</sub>, and
/// y = y<sub>1</sub>.
///
/// !["Ray rectangle intersection"][rayrectinter]
///
/// To determine whether a ray hits such a rectangle, we first determine where the ray hits the plane.
/// Recall that a ray **P**(t) = **A** + t**b** has its z component defined by
/// *P<sub>z</sub>(t) = A<sub>z</sub> + tb<sub>z</sub>*. Rearranging those terms we can solve for what
/// the t is where z = k.
///
/// ```math
///     t = \frac{k - A_z}{b_z}
/// ```
///
/// Once we have t, we can plug that into the equations for *x* and *y*:
///
/// ```math
///     x = A_x + tb_x
///     y = A_y + tb_y
/// ```
///
/// It is a hit if x<sub>0</sub> < x < x<sub>1</sub> and y<sub>0</sub> < y < y<sub>1</sub>.
///
/// Because our rectangles are axis-aligned, their bounding boxes will have an infinitely thin-side.
/// This can be a problem when dividing them up with our axis-aligned bounding volume hierarchy.
/// To counter this, all hittable objects should get a bounding box that has finite width along
/// every dimension. For our rectangles, we'll just pad the box a bit on the infinitely-thin side.
#[embed_doc_image("rayrectinter", "doc_images/ray_rectangle_intersection.jpg")]
pub struct XYRect {
    material: Rc<dyn Material>,
    x0: f64,
    x1: f64,
    y0: f64,
    y1: f64,
    k: f64,
}

impl Hittable for XYRect {
    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        // The bounding box must have non-zero width in each dimension, so pad the Z dimension a small amount
        Some(AABB::new(
            Point::new(self.x0, self.y0, self.k - 0.0001),
            Point::new(self.x0, self.y0, self.k + 0.0001),
        ))
    }

    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let t = (self.k - r.origin().z()) / r.direction().z();
        if t < t_min || t > t_max {
            return None;
        }
        let x = r.origin().x() + t * r.direction().x();
        let y = r.origin().y() + t * r.direction().y();
        if x < self.x0 || x > self.x0 || y < self.y0 || y < self.y1 {
            return None;
        }

        let mut rec = HitRecord {
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            t,
            mat: self.material.clone(),
            p: r.at(t),
            normal: Vec3::default(),
            front_face: false,
        };

        let outward_normal = Vec3::new(0.0, 0.0, 1.0);
        rec.set_face_normal(r, &outward_normal);

        Some(rec)
    }
}
