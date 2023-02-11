use crate::{
    utils::{random_in_unit_interval, random_int},
    Point,
};
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
pub(in crate::textures) struct Perlin {
    ranfloat: Vec<f64>,
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
}

impl Perlin {
    const POINT_COUNT: i32 = 256;

    fn perlin_generate_perm() -> Vec<i32> {
        let mut p: Vec<i32> = (0..Self::POINT_COUNT).collect();
        Self::permute(p.as_mut_slice());

        p
    }

    fn permute(p: &mut [i32]) {
        for i in (0..p.len()).rev() {
            let target = random_int(0, i as i32) as usize;
            p.swap(i, target);
        }
    }

    pub fn new() -> Perlin {
        let ranfloat: Vec<f64> = (0..Self::POINT_COUNT)
            .map(|_| random_in_unit_interval())
            .collect();
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        Perlin {
            ranfloat,
            perm_x,
            perm_y,
            perm_z,
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        let i = ((4.0 * p.x()) as i32 & 255) as usize;
        let j = ((4.0 * p.y()) as i32 & 255) as usize;
        let k = ((4.0 * p.z()) as i32 & 255) as usize;

        let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
        self.ranfloat[index as usize]
    }
}
