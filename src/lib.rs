mod camera;
mod ray;
mod rtweekend;
mod vec3;

use embed_doc_image::embed_doc_image;
use objects::{Hittable, HittableList};
// Export all the functions structs and constants for use in other crates.
pub mod materials;
pub mod objects;
pub use camera::Camera;
pub use ray::Ray;
pub use rtweekend::*;
pub use vec3::{Color, Point, Vec3};

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
/// ![Camera Geometry][camgeom]
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
pub fn ray_color(r: &Ray, world: &HittableList, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // # Fixing the shadow Acne
    // Some of the reflected rays hit the object they are reflecting off of not at exactly at
    // **t = 0**, but instead at **t = -0.0000001** or **t = 0.0000001** or whatever floating
    // point approximation the sphere intersector gives us. So we need to ignore hits very near zero:
    // So pass the t_min as 0.001.
    if let Some(hit_rec) = world.hit(r, 0.001, INFINITY) {
        // todo!("Use a strategy pattern to choose between different diffusers");
        //let target = hit_rec.p + hit_rec.normal + Vec3::random_vector_in_unit_sphere();
        // let target =
        //     hit_rec.p + hit_rec.normal + Vec3::random_unit_vector_lambertian_distribution();
        // let target = hit_rec.p + Vec3::random_unit_vector_in_hemisphere(&hit_rec.normal);
        // return 0.5
        //     * ray_color(
        //         &Ray::new(&hit_rec.p, &(target - hit_rec.p)),
        //         world,
        //         depth - 1,
        //     );

        if let Some((scattered, attenuation)) = hit_rec.mat.scatter(r, &hit_rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }

        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
