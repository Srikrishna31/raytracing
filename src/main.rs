use raytracing::{
    Color, Hittable, HittableList, IntersectionInterval, Point, Ray, Sphere, Vec3, INFINITY,
};
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{Result, Write};

fn main() {
    write_image()
}

fn write_color<T: Write>(out: &mut T, pixel_color: &Color) -> Result<usize> {
    let mut str = String::new();
    writeln!(
        str,
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32,
    )
    .expect("Error formatting write");

    out.write(str.as_bytes())
}

fn write_image() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = 400;

    // World
    let mut world = HittableList::new();
    world.add(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0));

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
            let pixel_color = ray_color(&r, &world);

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
fn ray_color(r: &Ray, world: &HittableList) -> Color {
    if let Some(hit_rec) = world.hit(r, &IntersectionInterval::new(0.0, INFINITY).unwrap()) {
        return 0.5 * (hit_rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
