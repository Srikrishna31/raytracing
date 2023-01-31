mod hittable;
mod hittablelist;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{Result, Write};

// Export all the functions and constants to other crates.
pub use ray::Ray;
pub use rtweekend::*;
pub use sphere::Sphere;
pub use vec3::{Color, Point, Vec3};

pub fn write_color<T: Write>(out: &mut T, pixel_color: &vec3::Color) -> Result<usize> {
    let mut str = String::new();
    write!(
        str,
        "{} {} {}\n",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32,
    )
    .expect("Error formatting write");

    out.write(str.as_bytes())
}

pub fn write_image() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = 400;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    // Render
    println!("P3\n{} {}\n255\n", &IMAGE_WIDTH, &IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", &j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                &origin,
                &(lower_left_corner + u * horizontal + v * vertical - origin),
            );
            let pixel_color = ray_color(&r);

            write_color(&mut io::stdout(), &pixel_color).expect("Error writing to output");
        }
        eprintln!("\nDone.\n")
    }
}

/// At the core, the ray tracer sends rays through pixels and computes the color seen in the direction
/// of those rays. The involved steps are (1) calculate the ray from the eye to the pixel, (2) determine
/// which objects the ray intersects, and (3) compute a color for that intersection point.
///
/// A common trick used for visualizing normals (because it's easy and somewhat intuitive to assume
/// **n** is a unit length vector - so each component is between -1 and 1) is to map each component
/// to the interval from 0 to 1, and then map x/y/z to r/g/b.
pub fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Point::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let N = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).unit_vector();
        return Color::new(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0) * 0.5;
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

/// # Ray-Sphere Intersection
/// The equation for a sphere centered at the origin of radius **R** is *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> = R<sup>2</sup>*
/// Put another way, if a given point (x,y,z) is on the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> = R<sup>2</sup>*.
/// If the given point is *inside* the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> < R<sup>2</sup>*,
/// and if a given point is *outside* the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> >R<sup>2</sup>*. It gets
/// uglier if the sphere center is at (C<sub>x</sub>, C<sub>y</sub>, C<sub>z</sub>):
///
/// *(x - C<sub>x</sub>)<sup>2</sup> + (y - C<sub>y</sub>)<sup>2</sup> + (z - C<sub>z</sub>)<sup>2</sup> = r<sup>2</sup>*
///
/// In graphics, you almost always want your formulas to be in terms of vectors so all the *x/y/z* stuff
/// is under the hood in the `Vec3` class. The vector from center **C**=(C<sub>x</sub>,C<sub>y</sub>,C<sub>z</sub>)
/// to point **P** = (*x,y,z*) is (**P** - **C**), and therefore
///
/// (**P** - **C**) . (**P** - **C**) = (*x - C<sub>x</sub>*)<sup>2</sup> + (*y - C<sub>y</sub>*)<sup>2</sup> + (*z - C<sub>z</sub>*)<sup>2</sup>.
///
/// So,the equation of the sphere in the vector form is:
///
/// (**P - C**).(**P - C**) = r<sup>2</sup>
///
/// We can read this as "any point **P** that satisfies this equation is on the sphere". We want to
/// know if our ray **P**(*t*) = **A** + *t***b** ever hits the sphere anywhere. If it does hit the
/// sphere, there is some *t* for which **P**(*t*) satisfies the sphere equation. So we are looking
/// for any *t* where this is true:
///
/// (**P**(*t*) - **C**).(**P**(*t*) - **C**) = r<sup>2</sup>
///
/// or expanding the full form of the ray **P**(*t*):
///
/// (**A** + *t***b** - **C**).(**A** + *t***b** - **C**) = *r*<sup>2</sup>
///
/// The rules of vector algebra are all that we want here. If we expand that equation and move all
/// the terms to the left hand side we get:
///
/// *t<sup>2</sup>***b**.**b** + 2*t***b**.(**A - C**) + (**A - C**).(**A - C**) - r<sup>2</sup> = 0
///
/// The vectors and *r* in that equation are all constant and known. The unknown is t, and the equation
/// is a quadratic. You can solve for *t* and there is a square root part that is either positive
/// (meaning two real solutions), negative (meaning no real solutions), or zero (meaning one real
/// solution).
fn hit_sphere(center: Point, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = ray.direction().dot(&ray.direction());
    let half_b = oc.dot(&ray.direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-half_b - f64::sqrt(discriminant)) / a
    }
}
