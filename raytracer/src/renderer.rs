use crate::{
    configuration::ImageFormat as ConfImageFormat,
    configuration::ImageSettings,
    objects::{BVHNode, Hittable},
    utils,
    utils::{clamp, random_in_unit_interval},
    Color, Ray, Scene,
};
use embed_doc_image::embed_doc_image;
use image::{ImageFormat, RgbaImage};
use rayon::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

/// To handle the multi-sampled color computation - rather than adding in a fractional contribution
/// each time we accumulate more light to the color, just add the full color each iteration, and
/// then perform a single divide at the end (by the number of samples) when writing out the color.
#[inline]
fn write_color(pixel: &mut [u8], pixel_color: &Color, samples_per_pixel: u32) {
    // Divide the color by the number of samples and gamma correct for gamma = 2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    let r = f64::sqrt(scale * pixel_color.x());
    let g = f64::sqrt(scale * pixel_color.y());
    let b = f64::sqrt(scale * pixel_color.z());

    // Write the translated [0,255] value of each color component
    pixel[0] = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    pixel[1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    pixel[2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;
    pixel[3] = 255;
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
/// Callbacks are of different types for each one defined, which is why they are behind a 'dyn'.
/// And since they have to be behind a dyn, it has to be encapsulated in a Box or an Rc.
/// The alternative is to make the callback as part of the function signature, and make it generic
/// over the callback type.
#[embed_doc_image("pixelsamples", "doc_images/pixel_samples.jpg")]
pub fn render<F>(settings: ImageSettings, scene: Scene, progress_callback: F)
where
    F: Fn(f64) + Sync + Send,
{
    // World and Camera
    let Scene {
        world,
        camera,
        background_color,
    } = scene;
    let bvh_world = Arc::new(BVHNode::new(&world, 0.0, 0.0).unwrap());
    let progress_counter = AtomicU64::new(0);

    // Render
    let iters: u32 = settings.width * settings.height;
    let mut imout = RgbaImage::new(settings.width, settings.height);
    // Since it's an rgba image, iterate in chunks of 4 (RGBA).
    imout
        .par_chunks_mut(4)
        .enumerate()
        .into_par_iter()
        .for_each(|(i, chk)| {
            let x = i % settings.width as usize;
            // Starting y from the beginning results in an inverted image, so start from the bottom
            // and work the way up.
            let y = settings.height as usize - 1 - i / settings.width as usize;
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..settings.samples_per_pixel {
                let u = (x as f64 + random_in_unit_interval()) / (settings.width - 1) as f64;
                let v = (y as f64 + random_in_unit_interval()) / (settings.height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color +=
                    ray_color(&r, &background_color, bvh_world.clone(), settings.max_depth);
            }

            let prev_value = progress_counter.fetch_add(1, Ordering::SeqCst);
            // Call the callback only on the boundaries of 10 pixels to avoid insignificant updates.
            if prev_value % 10 == 0 || iters < 10 {
                progress_callback((prev_value + 1) as f64 / iters as f64 * 100.0);
            }

            write_color(chk, &pixel_color, settings.samples_per_pixel);
        });

    imout
        .save_with_format(
            std::path::Path::new(&settings.path),
            get_format(settings.format),
        )
        .expect("Unable to save image in specified format");
}

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
///
/// ## Adding Background Color to the Ray Color Function
/// We want to be able to set a background color (probably black in presence of lights), so the only
/// light in the scene is coming from the emitters.
#[embed_doc_image("camgeom", "doc_images/camera_geometry.jpg")]
fn ray_color(r: &Ray, bg_color: &Color, world: Arc<dyn Hittable>, depth: u32) -> Color {
    // If we've exceeded the ray bounce limit, no more light is gathered.
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // # Fixing the shadow Acne
    // Some of the reflected rays hit the object they are reflecting off of not at exactly at
    // **t = 0**, but instead at **t = -0.0000001** or **t = 0.0000001** or whatever floating
    // point approximation the sphere intersector gives us. So we need to ignore hits very near zero:
    // So pass the t_min as 0.001.
    match world.hit(r, 0.001, utils::INFINITY) {
        Some(hit_rec) => {
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

            let emitted = hit_rec.mat.emitted(hit_rec.u, hit_rec.v, &hit_rec.p);
            match hit_rec.mat.scatter(r, &hit_rec) {
                Some((scattered, attenuation)) => {
                    emitted + attenuation * ray_color(&scattered, bg_color, world, depth - 1)
                }
                None => emitted,
            }
        }
        // If the ray hits nothing, return the background color
        None => *bg_color,
    }
}

fn get_format(format: ConfImageFormat) -> ImageFormat {
    match format {
        ConfImageFormat::Jpg => ImageFormat::Jpeg,
        ConfImageFormat::Png => ImageFormat::Png,
        ConfImageFormat::Tiff => ImageFormat::Tiff,
        ConfImageFormat::Ppm => ImageFormat::Pnm,
    }
}
