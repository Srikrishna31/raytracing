use embed_doc_image::embed_doc_image;

/// To get cool looking solid textures most people use some form of Perlin noise. These are named
/// after their inventor Ken Perlin. Perlin texture doesn't return white noise like this:
///
/// !["White Noise"][whitenoise]
///
/// Instead it returns something similar to blurred white noise:
///
/// !["White Noise, blurred"][whitenoiseblurred]
///
/// A key part of Perlin noise is that it is repeatable: it takes a 3D point as input and always returns
/// the same randomish number. Nearby points return similar numbers. Another important part of Perlin
/// noise is that it be simple and fast, so it's usually done as a hack.
#[embed_doc_image("whitenoise", "doc_images/white_noise.jpg")]
#[embed_doc_image("whitenoiseblurred", "doc_images/white_noise_blurred.jpg")]
pub struct Perlin {

}
