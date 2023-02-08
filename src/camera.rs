use crate::ray::Ray;
use crate::utils::degrees_to_radians;
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
/// This implies $h = tan(θ / 2)$.
///
/// # Positioning and Orienting the Camera
///
/// To get an arbitrary viewpoint, let's first name the points we care about. The position where the
/// camera is placed is called as *lookfrom*, and the point it looks at *lookat*. (Later, it can be
/// changed to a direction to look in instead of a point to look at.)
///
/// We also need a way to specify the roll, or sideways tilt, of the camera: the rotation around the
/// lookat-lookfrom axis. Another way to think about it is that even if you keep `lookfrom` and
/// `lookat` constant, you can still rotate your head around your nose. What we need is a way to
/// specify an "up" vector for the camera. This up vector should lie in the plane orthogonal to the
/// view direction.
///
/// ![Camera view direction][camviewdirection]
///
/// We can actually use any up vector we want, and simply project it onto this plane to get an up
/// vector for the camera. The common convention is to name "view up" (vup) vector. A couple of cross
/// products, and we now have a complete orthonormal basis ***(u,v,w)*** to describe our camera's
/// orientation.
///
/// ![Camera view up direction][camupdirection]
///
/// Remember that `vup`, `v`, and `w` are all in the same plane. Note that like before when our fixed
/// camera faced -Z, our arbitrary view camera faces -w. And  keep in mind that we can - but we don't
/// have to - use world up (0,1,0) to specify vup. This is convenient and will naturally keep your
/// camera horizontally level until you decide to experiment with crazy camera angles.
///
/// # Defocus Blur
///
/// The reason we defocus blur in real cameras is because they need a big hole (rather than just a
/// pinhole) to gather light. This would defocus everything, but if we stick a lens in the hole, there
/// will be a certain distance where everything is in focus. You can think of a lens this way: all
/// light rays coming from a specific point at the focus distance - and that hit the lens - will be
/// bent back to a single point on the image sensor.
///
/// We call the distance between the projection point and the plane where everything is in perfect
/// focus the *focus distance*. Be aware that the focus distance is not the same as the focal length
/// -- the *focal length* is the distance between the projection point and the image plane.
///
/// In a physical camera, the focus distance is controlled by the distance between the lens and the
/// film/sensor. That is why you see the lens move relative to the camera when you change what is in
/// focus (that may happen in your phone camera too, but the sensor moves). The "aperture" is a hole
/// to control how big the lens is effectively. For a real camera, if you need more light you make the
/// aperture bigger, and will get more defocus blur. For our virtual camera, we can have a perfect
/// sensor and never need more light, so we only have an aperture when we want defocus blur.
///
/// ## Thin lens approximation
/// A real camera has a complicated compound lens. For our code we could simulate the order: sensor,
/// then lens, then aperture. Then we could figure out where to send the rays, and flip the image after
/// it's computed (the image is projected upside down on the film). Graphics people, however, usually
/// use a thin lens approximation:
///
/// ![Camera lens model][camlensmodel]
///
/// We don't need to simulate any of the inside of the camera. Instead, we usually start rays from the
/// lens, and send them toward the focus plane (`focus_dist` away from the lens), where everything on
/// that plane is in perfect focus.
///
/// ![Camera focus plane][camfocusplane]
///
/// ## Generating Sample Rays
/// Normally, all scene rays originiate from the `lookfrom` point. In order to accomplish defocus blur,
/// generate random scene rays originating from inside a disk centered at the `lookfrom` point. The
/// larger the radius, the greater the defocus blur. You can think of our original camera as having α
/// defocus disk of radius zero (no blur at all), so all rays originated at the disk center (`lookfrom`).
#[embed_doc_image("camgeometry", "doc_images/camera_viewing_geometry.jpg")]
#[embed_doc_image("camviewdirection", "doc_images/camera_view_direction.jpg")]
#[embed_doc_image("camupdirection", "doc_images/camera_view_up_direction.jpg")]
#[embed_doc_image("camlensmodel", "doc_images/camera_lens_model.jpg")]
#[embed_doc_image("camfocusplane", "doc_images/camera_focus_plane.jpg")]
#[derive(Clone)]
pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    _w: Vec3,
    lens_radius: f64,
}

impl Camera {
    /// Returns a camera object with the given settings
    /// # Arguments
    /// * `vfov`: Vertical Field of View in Degrees
    /// todo: Encapsulate fov into a degrees type, to make the code more readable.
    /// * `aspect_ratio`: Aspect Ratio determines the width/length of the viewport.
    ///
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vector();
        let u = vup.cross(&w).unit_vector();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;

        Camera {
            origin,
            horizontal,
            vertical,
            lower_left_corner: origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w,
            lens_radius: aperture / 2.0,
            u,
            v,
            _w: w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * Vec3::random_vector_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();

        Ray::new(
            &(self.origin + offset),
            &(self.lower_left_corner + s * self.horizontal + t * self.vertical
                - self.origin
                - offset),
        )
    }
}

impl Default for Camera {
    /// This function returns a camera positioned at (0,0,0), looking at (0,0,-1), with up vector
    /// (0,1,0), 90 degree field of view, 16:9 aspect raio and aperture 0 (no blur).
    fn default() -> Self {
        let lookfrom = Vec3::new(0.0, 0.0, 0.0);
        let lookat = Vec3::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = (lookfrom - lookat).length();
        let aperture = 0.0;
        let aspect_ratio = 16.0 / 9.0;
        let fov = 90.0;

        Camera::new(
            lookfrom,
            lookat,
            vup,
            fov,
            aspect_ratio,
            aperture,
            dist_to_focus,
        )
    }
}
