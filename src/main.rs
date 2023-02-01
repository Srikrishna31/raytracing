use raytracing::{
    clamp, random_in_unit_interval, Camera, Color, Hittable, HittableList, Point, Ray, Sphere,
    INFINITY,
};
use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{Result, Write};

fn main() {
    write_image()
}

fn write_color<T: Write>(
    out: &mut T,
    pixel_color: &Color,
    samples_per_pixel: i32,
) -> Result<usize> {
    let mut r = pixel_color.z();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples.
    let scale = 1.0 / samples_per_pixel as f64;
    r *= scale;
    g *= scale;
    b *= scale;

    // Write the translated [0,255] value of each color component
    let mut str = String::new();
    writeln!(
        str,
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32,
    )
    .expect("Error formatting write");

    out.write(str.as_bytes())
}

fn write_image() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = 400;
    const SAMPLES_PER_PIXEL: i32 = 100;

    // World
    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::new();

    // Render
    println!("P3\n{} {}\n255\n", &IMAGE_WIDTH, &IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("\rScanlines remaining: {} ", &j);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_in_unit_interval()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_in_unit_interval()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world);
            }

            write_color(&mut io::stdout(), &pixel_color, SAMPLES_PER_PIXEL)
                .expect("Error writing to output");
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
    if let Some(hit_rec) = world.hit(r, 0.0, INFINITY) {
        return 0.5 * (hit_rec.normal + Color::new(1.0, 1.0, 1.0));
    }
    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
