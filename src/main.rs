mod scenes;

use embed_doc_image::embed_doc_image;
use raytracing::{
    ray_color,
    utils::{clamp, random_in_unit_interval},
    Color,
    load_configuration,
    ImageSettings
};

use std::fmt::Write as FmtWrite;
use std::io;
use std::io::{Result, Write};
use std::rc::Rc;

/// Callbacks are of different types for each one defined, which is why they are behind a 'dyn'.
/// And since they have to be behind a dyn, it has to be encapsulated in a Box or an Rc.
/// The alternative is to make the callback as part of the function signature, and make it generic
/// over the callback type. That is an overkill for the rendering/write_image function.
pub type ProgressCallback = Rc<dyn Fn(f64) -> ()>;

fn main(){
    let cb: ProgressCallback = Rc::new(|i: f64| eprintln!("{i:.2}% completed"));
    let settings = load_configuration().expect("Couldnot read settings");
    write_image(settings, cb)
}

/// To handle the multi-sampled color computation - rather than adding in a fractional contribution
/// each time we accumulate more light to the color, just add the full color each iteration, and
/// then perform a single divide at the end (by the number of samples) when writing out the color.
fn write_color<T: Write>(
    out: &mut T,
    pixel_color: &Color,
    samples_per_pixel: i32,
) -> Result<usize> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma correct for gamma = 2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

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

/// # Antialiasing
/// When a real camera takes a picture, there are usually no jaggies along edges because the edge
/// pixels are a blend of some foreground and some background. We can get the same effect by averaging
/// a bunch of samples inside each pixel. We will not bother with stratification.
///
/// For a given pixel we have several samples within that pixel and send rays through each of the
/// samples. The colors of these rays are then averaged:
///
/// ![Pixel Samples][pixelsamples]
///
#[embed_doc_image("pixelsamples", "doc_images/pixel_samples.jpg")]
fn write_image(settings: ImageSettings, cb: ProgressCallback) {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: u32 = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as u32;
    const IMAGE_HEIGHT: u32 = 400;
    const SAMPLES_PER_PIXEL: i32 = 100;
    const MAX_DEPTH: u32 = 50;

    // const ASPECT_RATIO: f64 = 3.0 / 2.0;
    // const IMAGE_WIDTH: u32 = 1200;
    // const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
    // const SAMPLES_PER_PIXEL: i32 = 500;
    // const MAX_DEPTH: u32 = 50;

    // World and Camera
    let (world, camera) = scenes::scene_with_alternate_viewpoint();

    // Render
    println!("P3\n{} {}\n255\n", &IMAGE_WIDTH, &IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        cb((1.0 - j as f64 / IMAGE_HEIGHT as f64) * 100.0);
        for i in 0..IMAGE_WIDTH {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_in_unit_interval()) / (IMAGE_WIDTH - 1) as f64;
                let v = (j as f64 + random_in_unit_interval()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, MAX_DEPTH);
            }

            write_color(&mut io::stdout(), &pixel_color, SAMPLES_PER_PIXEL)
                .expect("Error writing to output");
        }
    }
    eprintln!("\nDone.\n")
}
