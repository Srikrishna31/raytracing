use crate::objects::{HitRecord, Hittable, AABB};
use crate::utils::{degrees_to_radians, INFINITY};
use crate::{Point, Ray, Vec3};
use embed_doc_image::embed_doc_image;
use std::rc::Rc;

/// # Instance Rotation
/// A common graphics tactic is to apply all rotations about the x, y and z axes. These rotations are
/// in some sense axis-aligned. First let's rotate by theta about the z-axis. That will be changing
/// only x and y, and in ways that don't depend on z.
///
/// !["Rotation about the z axis"][zrotation]
///
/// The result for rotating counter-clockwise about z is:
///
/// ```math
///         x^' = cos(θ) . x - sin(θ) . y
///         y^' = sin(θ) . x + cos(θ) . y
/// ```
///
/// The great thing is that it works for any θ and doesn't need any cases for quadrants or anything
/// like that. The inverse transform is the opposite geometric operation: rotate by -θ. Here recall
/// that cos(θ) = cos(-θ) and sin(-θ) = -sin(θ), so the formulas are very simple.
///
/// Similarly, for rotating about y, the formulas are:
///
/// ```math
///         x^' = cos(θ) . z + sin(θ) . x
///         z^' = -sin(θ) . x + cos(θ) . x
/// ```
///
/// And about the x-axis:
///
/// ```math
///         y^' = cos(θ) . y - sin(θ) . z
///         z^' = sin(θ) . y + cos(θ) . z
/// ```
///
/// Unlike the situation with translations, surface normal vector also changes, so we need to transform
/// directions too if we get a hit. Fortunately, for rotations, the same formulas apply.
///
#[embed_doc_image("zrotation", "doc_images/rotation_about_z_axis.jpg")]
pub struct RotateY {
    sin_theta: f64,
    cos_theta: f64,
    bbox: Option<AABB>,
    obj: Rc<dyn Hittable>,
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut origin = r.origin();
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z();
        origin[2] = self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z();

        direction[0] = self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z();
        direction[2] = self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z();

        let rotated_ray = Ray::new_with_time(&origin, &direction, r.time());

        self.obj.hit(&rotated_ray, t_min, t_max).map(|mut hit_rec| {
            let p = hit_rec.p;
            let normal = hit_rec.normal;

            p[0] = self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z();
            p[2] = -self.sin_theta * rec.p.z() + self.cos_theta * rec.p.z();

            normal[0] = self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z();
            normal[2] = -self.sin_theta * rec.normal.z() + self.cos_theta * rec.normal.z();

            hit_rec.p = p;
            hit_rec.set_face_normal(&rotated_ray, &normal);

            hit_rec
        })
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        self.bbox.clone()
    }
}

impl RotateY {
    pub fn new(obj: Rc<dyn Hittable>, angle_degrees: f64) -> RotateY {
        let radians = degrees_to_radians(angle_degrees);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();

        let bbox = obj.bounding_box(0.0, 1.0).map(|obj_box| {
            let mut min = Point::new(INFINITY, INFINITY, INFINITY);
            let mut max = Point::new(-INFINITY, -INFINITY, -INFINITY);

            for i in 0..2 {
                for j in 0..2 {
                    for k in 0..2 {
                        let ii = i as f64;
                        let jj = j as f64;
                        let kk = k as f64;

                        let x = ii * obj_box.max().x() + (1.0 - ii) * obj_box.min().x();
                        let y = jj * obj_box.max().y() + (1.0 - jj) * obj_box.min().y();
                        let z = kk * obj_box.max().z() + (1.0 - kk) * obj_box.min().z();

                        let newx = cos_theta * x + sin_theta * z;
                        let newz = -sin_theta * x + cos_theta * z;

                        let tester = Vec3::new(newx, y, newz);

                        for l in 0..3 {
                            min[l] = min[l].min(tester[l]);
                            max[l] = max[l].max(tester[l]);
                        }
                    }
                }
            }

            AABB::new(min, max)
        });

        RotateY {
            cos_theta,
            sin_theta,
            bbox,
            obj,
        }
    }
}
