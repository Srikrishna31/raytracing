use crate::objects::{Hittable, AABB};
use std::rc::Rc;
use embed_doc_image::embed_doc_image;

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
struct BVHNode {
    left: Rc<dyn Hittable> ,
    right: Rc<dyn Hittable>,
    bbox: AABB
}
