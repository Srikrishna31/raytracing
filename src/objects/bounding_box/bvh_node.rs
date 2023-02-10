use crate::objects::{HitRecord, Hittable, AABB};
use crate::utils::random_int;
use crate::Ray;
use embed_doc_image::embed_doc_image;
use std::cmp::Ordering;
use std::rc::Rc;

/// # Bounding Volume Hierarchies
/// The ray-object intersection is the main time-bottleneck in a ray tracer, and the time is linear
/// with the number of objects. But it's a repeated search on the same model, so we ought to be able
/// to make it a logarithmic search in the spirit of binary search. Because we are sending millions
/// to billions of rays on the same model, we can do an analog of sorting the model, and then each
/// ray intersection can be a sublinear search. The two most common families of sorting are to
/// * divide the space
/// * divide the objects
///
/// The latter is usually much easier to code up and just as fast to run for most models.
///
/// ## The Key Idea
/// The key idea of a bounding volume over a set of primitives is to find a volume that fully
/// encloses (bounds) all the objects. For example, suppose you computed a bounding sphere of 10
/// objects. Any ray that misses the bounding sphere definitely misses all ten objects. If the ray
/// hits the bounding sphere, then it might hit one of the ten objects. So the bounding code is always
/// of the form:
///
/// ```code
///     if (ray hits bounding object)
///         return whether ray hits bounded objects
///     else
///         return false
/// ```
///
/// A key thing is we are dividing objects into subsets. We are not dividing the screen or the volume.
/// Any object is in just one bounding volume, but bounding volumes can overlap.
///
/// ## Hierarchies of Bounding Volumes
/// To make things sub-linear we need to make the bounding volumes hierarchical. For example, if we
/// divided a set of objects into two groups, red and blue, and used rectangular bounding volumes,
/// we'd have:
///
/// !["Bounding Volume Hierarchy"][bvh]
///
/// Note that the blue and red bounding volumes are contained in the purple one, but they might
/// overlap, and they are not ordered -- they are just both inside. So the tree shown on the right
/// has no concept of ordering in the left and right children; they are simply inside. The code would be:
///
/// ```code
///     if (hits purple)
///         hit0 = hits blue enclosed objects
///         hit1 = hits red enclosed objects
///         if (hit0 or hit1)
///             return true and info of closer hit
///     return false
/// ```
#[embed_doc_image("bvh", "doc_images/bounding_volume_hierarchy.jpg")]
pub(in crate::objects) struct BVHNode {
    left: Rc<dyn Hittable>,
    right: Rc<dyn Hittable>,
    bbox: AABB,
}

impl Hittable for BVHNode {
    /// A BVH is also going to be a `hittable` -- just like lists of `hittables`. It's really a container
    /// but it can respond to the query "does this ray hit you?".
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        if !self.bbox.hit(r, t_min, t_max) {
            return None;
        }

        let left = self.left.hit(r, t_min, t_max);
        let right = self
            .right
            .hit(r, t_min, left.as_ref().map_or_else(|| t_max, |v| v.t));

        match (left, right) {
            (None, None) => None,
            (Some(lbox), None) => Some(lbox),
            (None, Some(rbox)) => Some(rbox),
            (Some(_lbox), Some(rbox)) => Some(rbox),
        }
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(self.bbox.clone())
    }
}

impl BVHNode {
    /// # Splitting BVH Volumes
    /// The most complicated part of any efficiency structure, including the BVH, is building it. We
    /// do this in the constructor. A cool thing about BVHs is that as long as the list of objects in
    /// a `bvh_node` gets divided into two sub-lists, the hit function will work. It will work best
    /// if the division is done well, so that the two children have smaller bounding boxes than their
    /// parent's bounding box, but that is for speed not correctness. We'll choose the following approach:
    /// 1. randomly choose an axis
    /// 2. sort the primitives
    /// 3. put half in each subtree
    ///
    /// When the list coming in is two elements, we put one in each subtree and end the recursion. The
    /// traversal algorithm should be smooth and not have to check for null pointers, so if we get a
    /// list with just one element, we duplicate it in each subtree.
    pub fn new(
        src_objects: &mut Vec<Rc<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> Result<BVHNode, String> {
        Self::new_helper(src_objects, 0, src_objects.len(), time0, time1)
    }

    fn new_helper(
        src_objects: &mut Vec<Rc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> Result<BVHNode, String> {
        let mut objects = src_objects.clone();
        let axis = random_int(0, 2) as u8;
        let comparator = |a: &Rc<dyn Hittable>, b: &Rc<dyn Hittable>| {
            BVHNode::box_compare(a.clone(), b.clone(), axis).unwrap()
        };
        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(&objects[start], &objects[start + 1]) == Ordering::Less {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects.as_mut_slice()[start..end].sort_by(comparator);

            let mid = start + object_span / 2;

            // Will need to specify the type here as dyn Hittable. Otherwise, Rust treats as Rc<BVHNode>
            // and complains that the types don't match. Probably in future it might not be needed to do this.
            let left: Rc<dyn Hittable> =
                Rc::new(Self::new_helper(&mut objects, start, mid, time0, time1)?);
            let right: Rc<dyn Hittable> =
                Rc::new(Self::new_helper(&mut objects, mid, end, time0, time1)?);

            (left, right)
        };

        let lbox = left.bounding_box(time0, time1);
        let rbox = right.bounding_box(time0, time1);

        let bbox = match (lbox, rbox) {
            (Some(lb), Some(rb)) => AABB::surrounding_box(&lb, &rb),
            (_, _) => return Err("No bounding box in BVHNode constructor.\n".to_string()),
        };

        Ok(BVHNode { left, right, bbox })
    }

    #[inline]
    fn box_compare(a: Rc<dyn Hittable>, b: Rc<dyn Hittable>, axis: u8) -> Result<Ordering, String> {
        let axis = axis as usize;
        let (a_box, b_box) = match (a.bounding_box(0.0, 0.0), b.bounding_box(0.0, 0.0)) {
            (Some(ab), Some(bb)) => (ab, bb),
            (_, _) => return Err("No bounding box in Hittable".to_string()),
        };

        if a_box.min().e[axis] <= b_box.min().e[axis] {
            Ok(Ordering::Less)
        } else {
            Ok(Ordering::Greater)
        }
    }
}
