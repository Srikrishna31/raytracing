use embed_doc_image::embed_doc_image;
use std::io::Result;

use crate::{
    configuration::ImageFormat as ConfImageFormat,
    configuration::ImageSettings,
    objects::{BVHNode, Hittable},
    utils,
    utils::{clamp, random_in_unit_interval},
    Color, Ray, Scene,
};
use image::{ImageBuffer, ImageFormat};

/// To handle the multi-sampled color computation - rather than adding in a fractional contribution
/// each time we accumulate more light to the color, just add the full color each iteration, and
/// then perform a single divide at the end (by the number of samples) when writing out the color.
fn write_color(out: &mut [u8; 3], pixel_color: &Color, samples_per_pixel: u32) -> Result<usize> {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    // Divide the color by the number of samples and gamma correct for gamma = 2.0.
    let scale = 1.0 / samples_per_pixel as f64;
    r = f64::sqrt(scale * r);
    g = f64::sqrt(scale * g);
    b = f64::sqrt(scale * b);

    // Write the translated [0,255] value of each color component
    out[0] = (256.0 * clamp(r, 0.0, 0.999)) as u8;
    out[1] = (256.0 * clamp(g, 0.0, 0.999)) as u8;
    out[2] = (256.0 * clamp(b, 0.0, 0.999)) as u8;

    Ok(3)
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
    F: Fn(f64),
{
    // World and Camera
    let Scene {
        world,
        camera,
        background_color,
    } = scene;
    let bvh_world = BVHNode::new(&world, 0.0, 0.0).unwrap();

    // Render
    let mut imout = ImageBuffer::<image::Rgb<u8>, Vec<u8>>::new(settings.width, settings.height);
    for i in 0..settings.width * settings.height {
        let x = i / settings.width;
        let y = i % settings.width;
        let pixel =     imout.get_pixel_mut(x, y);
        let mut pixel_color = Color::new(0.0, 0.0, 0.0);
        for _ in 0..settings.samples_per_pixel {
            let u = (x as f64 + random_in_unit_interval()) / (settings.width - 1) as f64;
            let v = (y as f64 + random_in_unit_interval()) / (settings.height - 1) as f64;
            let r = camera.get_ray(u, v);
            pixel_color += ray_color(&r, &background_color, &bvh_world, settings.max_depth);
        }

        write_color(&mut pixel.0, &pixel_color, settings.samples_per_pixel)
            .expect("Error writing to output");

        progress_callback(y as f64 / settings.height as f64 * 100.0);
    }

    // for (i, j, pixel) in imout.enumerate_pixels_mut() {
    //     progress_callback(j as f64 / settings.height as f64 * 100.0);
    //     let mut pixel_color = Color::new(0.0, 0.0, 0.0);
    //     for _ in 0..settings.samples_per_pixel {
    //         let u = (i as f64 + random_in_unit_interval()) / (settings.width - 1) as f64;
    //         let v = (j as f64 + random_in_unit_interval()) / (settings.height - 1) as f64;
    //         let r = camera.get_ray(u, v);
    //         pixel_color += ray_color(&r, &background_color, &bvh_world, settings.max_depth);
    //     }
    //
    //     write_color(&mut pixel.0, &pixel_color, settings.samples_per_pixel)
    //         .expect("Error writing to output");
    // }

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
fn ray_color(r: &Ray, bg_color: &Color, world: &dyn Hittable, depth: u32) -> Color {
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
                    // eprintln!("Ray hit *****something*******!");
                    emitted + attenuation * ray_color(&scattered, bg_color, world, depth - 1)
                }
                None => emitted,
            }
        }
        // If the ray hits nothing, return the background color
        None => {
            // eprintln!("Ray hit nothing! Emitting background color");
            *bg_color
        }
    }
}

fn get_format(format: ConfImageFormat) -> ImageFormat {
    match format {
        ConfImageFormat::Jpg => ImageFormat::Jpeg,
        ConfImageFormat::Png => ImageFormat::Png,
        ConfImageFormat::Tiff => ImageFormat::Tiff,
        ConfImageFormat::Ppm => ImageFormat::Pnm,
        _ => {
            eprintln!("Unsupported format. Defaulting to JPEG");
            ImageFormat::Jpeg
        }
    }
}
