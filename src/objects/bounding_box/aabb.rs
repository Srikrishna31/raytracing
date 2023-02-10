use crate::{Point, Ray};
use embed_doc_image::embed_doc_image;

/// # Axis-Aligned Bounding Boxes (AABBs)
/// A ray bounding volume intersection needs to be fast, and bounding volumes need to be pretty
/// compact. In practice for most models, axis-aligned boxes work better than the alternatives, but
/// this design choice is always something to keep in mind if you encounter unusual type of models.
///
/// All we need to know about AABB is whether or not we hit it; we don't need hit points or any of
/// that stuff that we need for an object we want to display.
///
/// The "slab" method is based on the observation that an n-dimensional AABB is just the intersection
/// of n axis-aligned intervals, often called "slabs". An interval is just the points
/// between two endpoints, e.g., *x* such that **3 < x < 5**, or more succinctly *x* in **(3,5)**.
/// In 2D, two intervals overlapping makes a 2D AABB (a rectangle):
///
/// !["2D Axis Aligned Bounding Box"][2daabb]
///
/// For a ray to hit one interval we first need to figure out whether the ray hits the boundaries.
/// For example, again in 2D, this is the ray parameters *t<sub>0</sub>* and *t<sub>1</sub>*. (If the
/// ray is parallel to the plane those will be undefined).
///
/// !["Ray Slab Intersection"][rayslab]
///
/// In 3D, those boundaries are planes. The equations for the planes are ***x = x<sub>0</sub>***, and
/// ***x = x<sub>1</sub>***. Where does the ray hit that plane? Recall that the ray can be thought of
/// as just a function that given a *t* returns a location **P(*t*)**:
///
/// ```math
///     P(t) = A + tb
/// ```
///
/// That equation applies to all three of the x/y/z coordinates.
/// For example, x(t) = **A**<sub>x</sub> + *tb<sub>x</sub>*. This ray hits the plane ***x = x<sub>0</sub>***
/// at the *t* that satisfies this equation:
///
/// ```math
///     x_0 = A_x + t_0b_x
/// ```
///
/// Thus *t* at that hitpoint is:
///
/// ```math
///     t_0 = \frac{(x_0 - A_x)}{b_x}
/// ```
///
/// We get the similar expression for *x<sub>1</sub>*:
///
/// ```math
///     t_1 = \frac{(x1 - A_x)}{b_x}
/// ```
///
/// The key observation to turn that 1D math into a hit test is that for a hit, the *t*-intervals
/// need to overlap. For example, in 2D the green and blue overlapping only happens if there is a hit:
///
/// !["Ray-slab t-interval overlap"][rayslabinterval]
///
#[embed_doc_image("2daabb", "doc_images/2D_axis_aligned_bounding_box.jpg")]
#[embed_doc_image("rayslab", "doc_images/ray_slab_intersection.jpg")]
#[embed_doc_image("rayslabinterval", "doc_images/ray_slab_t_interval_overlap.jpg")]
#[derive(Clone)]
pub struct AABB {
    minimum: Point,
    maximum: Point,
}

impl AABB {
    pub fn new(minimum: Point, maximum: Point) -> AABB {
        AABB { minimum, maximum }
    }

    /// Given two boxes, computes the union of two boxes and returns the box that
    /// will contain the provided two boxes.
    pub fn surrounding_box(box1: &AABB, box2: &AABB) -> AABB {
        let small = Point::new(
            box1.min().x().min(box2.min().x()),
            box1.min().y().min(box2.min().y()),
            box1.min().z().min(box2.min().z()),
        );

        let big = Point::new(
            box1.max().x().max(box2.max().x()),
            box1.max().y().max(box2.max().y()),
            box1.max().z().max(box2.max().z()),
        );

        AABB::new(small, big)
    }

    pub fn min(&self) -> Point {
        self.minimum
    }

    pub fn max(&self) -> Point {
        self.maximum
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction()[a];
            let mut t0 = (self.min()[a] - r.origin()[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.origin()[a]) * inv_d;

            if inv_d < 0.0 {
                (t1, t0) = (t0, t1); // swap t0 and t1;
            }

            let t_min_temp = if t0 > t_min { t0 } else { t_min };
            let t_max_temp = if t1 < t_max { t1 } else { t_max };

            if t_max_temp <= t_min_temp {
                return false;
            }
        }

        true
    }
}
