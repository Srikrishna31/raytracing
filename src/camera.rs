use crate::degrees_to_radians;
use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use embed_doc_image::embed_doc_image;

/// # Camera Viewing Geometry
///
/// The rays are designed to be coming from the origin and heading to the $ z = -1 $ plane. We could
/// make it $ z = -2 $ plane, or whatever, as long as we make *h* a ratio to that distance. Below is
/// the setup:
///
/// ![Camera Viewing Geometry][camgeometry]
///
/// This implies $ h = tan(θ / 2) $.
#[embed_doc_image("camgeometry", "doc_images/camera_viewing_geometry.jpg")]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    /// Returns a camera object with the given settings
    /// # Arguments
    /// * `vfov`: Vertical Field of View in Degrees
    /// todo: Encapsulate fov into a degrees type, to make the code more readable.
    /// * `aspect_ratio`: Aspect Ratio determines the width/length of the viewport.
    ///
    pub fn new(vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Point::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin
                - horizontal / 2.0
                - vertical / 2.0
                - Vec3::new(0.0, 0.0, focal_length),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            &self.origin,
            &(self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin),
        )
    }
}
