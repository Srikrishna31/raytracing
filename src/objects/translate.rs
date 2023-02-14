use crate::objects::{HitRecord, Hittable, AABB};
use crate::{Ray, Vec3};
use embed_doc_image::embed_doc_image;
use std::rc::Rc;

/// # Instance Translation
///
/// An instance is a geometric primitive that has been moved or rotated somehow. This is especially
/// easy in raytracing because we don't move anything; instead we move the rays in the opposite
/// direction. For example, consider a *translation* (often called a move). We could take the pink
/// box at the origin and add 2 to all it's components, or (as we almost always do in raytracing),
/// leave the box where it is, but in its hit routine subtract 2 off the x-component of the ray origin.
///
/// !["Ray-box intersection with moved ray vs box"][rayrotatedbox]
#[embed_doc_image(
    "rayrotatedbox",
    "doc_images/ray_box_intersection_with_moved_ray_vs_box.jpg"
)]
pub struct Translate {
    obj: Rc<dyn Hittable>,
    offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let moved_ray = Ray::new_with_time(&(r.origin() - self.offset), &r.direction(), r.time());

        self.obj.hit(&moved_ray, t_min, t_max).map(|mut hit_rec| {
            hit_rec.p = self.offset;
            // Need to make a copy of normal here to avoid the compiler error:
            // cannot borrow `hit_rec` as mutable because it is also borrowed as immutable
            let normal = hit_rec.normal;
            hit_rec.set_face_normal(&moved_ray, &normal);
            hit_rec
        })
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.obj
            .bounding_box(time0, time1)
            .map(|bbox| AABB::new(bbox.min() + self.offset, bbox.max() + self.offset))
    }
}
