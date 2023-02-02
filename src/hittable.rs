use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use embed_doc_image::embed_doc_image;

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

/// This trait represents an object that can be hit by a ray and return a result that can be used for
/// shading or omitting the object from the scene.
pub trait Hittable {
    /// Returns a HitRecord object containing the details of point of impact, normal at the point of
    /// impact, position (t) on the ray and a bool indicating the front/back face, if the object
    /// implementing this trait is hit. Otherwise returns None.
    ///
    /// # Arguments
    /// * `r` - The ray against which the object is to be tested.
    /// * `t_min` - The minimum point of the ray
    /// * `t_max` - The maximum point of the ray
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3 { e: [0.0, 0.0, 0.0] },
            normal: Vec3 { e: [0.0, 0.0, 0.0] },
            t: 0.0,
            front_face: false,
        }
    }

    /// # Surface Normals
    /// A surface normal is a vector that is perpendicular to the surface at the point of intersection.
    /// There are two design decisions to make for normals. The first is whether these normals are
    /// unit length. That is convenient for shading, but it could allow subtle bugs, so this is more
    /// of a personal preference. For a sphere, the outward normal is in the direction of the hit point
    /// minus the center:
    ///
    /// ![Sphere surface-normal geometry][surfgeom]
    ///
    /// The second design decision for normals is whether they should always point out. If the ray
    /// intersects the sphere from outside, the normal points against the ray. If the ray intersects
    /// the sphere from inside, the normal (which always points out) points with the ray. Alternatively
    /// we can have the normal always point against the ray. If the ray is outside the sphere, the
    /// normal will point outward, but if the ray is inside the sphere, the normal will point inward.
    ///
    /// ![Possible Directions for Sphere Surface Normal Geometry][normsides]
    ///
    /// We need to choose one of these possibilities because we will eventually want to determine
    /// which side of the surface that the ray is coming from. This is important for objects that are
    /// rendered differently on each side, like the text on a two-sided sheet of paper, or for objects
    /// that have an inside and an outside, like glass balls.
    ///
    /// We can set things up so that normals always point "outward" from the surface, or always point
    /// against the incident ray. This decision is determined by whether you want to determine the
    /// side of the surface at the time of geometry intersection or at the time of coloring.
    #[embed_doc_image("surfgeom", "doc_images/sphere_surface_normal_geometry.jpg")]
    #[embed_doc_image(
        "normsides",
        "doc_images/possible_directions_for_sphere_surface_normal_geometry.jpg"
    )]
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            } else {
                -outward_normal
            }
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
