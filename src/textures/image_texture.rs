
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

}