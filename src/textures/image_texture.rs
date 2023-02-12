use std::path::Path;
use crate::{Color, Point};
use crate::utils::clamp;
use super::Texture;
use image;
use image::{DynamicImage, GenericImageView};

/// # Image Texture Mapping
/// From the point **P**, we compute the surface coordinates *(u,v)*. We then use these to index into
/// our procedural solid texture (like marble). We can also read in an image and use the 2D (u,v)
/// texture coordinate to index into the image.
///
/// A direct way to use scaled *(u,v)* in an image is to round the u and v to integers, and use that
/// as *(i,j)* pixels. This is awkward, because we don't want to have to change the code when we change
/// the image rsolution. So, instead one of the most universal unofficial standards in graphics is to
/// use texture coordinates instead of image pixel coordinates. These are just some form of fractional
/// position in the image. For example, for pixel *(i,j)* in an N<sub>x</sub> by N<sub>y</sub> image
/// the image texture position is:
///
/// ```math
///     u = \frac{i}{N_x - 1}
///     v = \frac{j}{N_y - 1}
/// ```
///
/// This is just a fractional position.
#[derive(Clone)]
pub struct ImageTexture {
    img: DynamicImage,
    // bytes_per_scanline: u32,
}

impl Texture for ImageTexture {
    fn value(&self, u: f64, v: f64, _p: &Point) -> Color {
        let (width, height) = (self.img.width(), self.img.height());
        // If we have no texture data, then return solid cyan as a debugging aid
        if height == 0 || width == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        // Clamp input texture coordinates to [0,1] x [1, 0]
        let u = clamp(u, 0.0, 1.0);
        let v = 1.0 - clamp(v, 0.0, 1.0); //Flip V to image coordinates.


        let i = {
            let i =(u * width as f64) as u32;
            // Clamp integer mapping since actual coordinates should be less than 1.0
            if i >= width {
                width - 1
            } else {
                i
            }
        };

        let j = {
            let j = (v * height as f64) as u32;
            if j >= height {
                height - 1
            } else {
                j
            }
        };

        let color_scale = 1.0 / 255.0;
        let pixel = self.img.get_pixel(i, j);

        //Currently we are not caring about alpha channel.
        Color::new(color_scale*pixel[0] as f64, color_scale*pixel[1] as f64, color_scale*pixel[2] as f64)
    }
}

impl ImageTexture {
    pub fn new(file: &Path) -> ImageTexture {
        let img = image::open(file).expect("File not found");
        //TODO: Write a log statement, which checks if the image is empty.
        ImageTexture {img}
    }
}
