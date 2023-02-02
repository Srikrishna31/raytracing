mod camera;
mod hittable;
mod hittablelist;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

// Export all the functions structs and constants for use in other crates.
pub use camera::Camera;
pub use hittable::Hittable;
pub use hittablelist::HittableList;
pub use ray::Ray;
pub use rtweekend::*;
pub use sphere::Sphere;
pub use vec3::{Color, Point, Vec3};

use embed_doc_image::embed_doc_image;

/// At the core, the ray tracer sends rays through pixels and computes the color seen in the direction
/// of those rays. The involved steps are (1) calculate the ray from the eye to the pixel, (2) determine
/// which objects the ray intersects, and (3) compute a color for that intersection point.
///
/// In addition to setting up the pixel dimensions for the rendered image, we also need to setup a
/// virtual viewport through which to pass our scene rays. For the standard square pixel spacing, the
/// viewport's aspect ratio should be the same as our rendered image. We'll just pick a viewport two
/// units in height. We'll also set the distance between the projection plane and the projection point
/// to be one unit. This is referred to as the "focal length"
///
/// ![Camera Geomety][camgeom]
///
/// The "eye" (or camera center if you think of a camera) is at (0,0,0). The y-axis is pointing upwards,
/// and the x-axis goes towards the right. In order to respect the convention of a right handed
/// coordinate system, into the screen is negative z-axis. The screen will be traversed from the upper
/// left hand corner, and two offset vectors will be used, along the screen sides to move the ray
/// endpoint across the screen.
///
/// A common trick used for visualizing normals (because it's easy and somewhat intuitive to assume
/// **n** is a unit length vector - so each component is between -1 and 1) is to map each component
/// to the interval from 0 to 1, and then map x/y/z to r/g/b.
#[embed_doc_image("camgeom", "doc_images/camera_geometry.jpg")]
pub fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(hit_rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (hit_rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
