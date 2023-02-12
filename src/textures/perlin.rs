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
    option: PerlinNoiseOptions,
}

#[derive(PartialEq)]
pub enum PerlinNoiseOptions {
    Default,
    TrilinearSmoothing,
    HermitianSmoothing,
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

    pub fn new(option: PerlinNoiseOptions) -> Perlin {
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
            option,
        }
    }

    pub fn noise(&self, p: &Point) -> f64 {
        match &self.option {
            PerlinNoiseOptions::Default => {
                let i = ((4.0 * p.x()) as i32 & 255) as usize;
                let j = ((4.0 * p.y()) as i32 & 255) as usize;
                let k = ((4.0 * p.z()) as i32 & 255) as usize;

                let index = self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k];
                self.ranfloat[index as usize]
            }
            PerlinNoiseOptions::TrilinearSmoothing | PerlinNoiseOptions::HermitianSmoothing => {
                let mut u = p.x() - p.x().floor();
                let mut v = p.y() - p.y().floor();
                let mut w = p.z() - p.z().floor();

                if self.option == PerlinNoiseOptions::HermitianSmoothing {
                    u = u * u * (3.0 - 2.0 * u);
                    v = v * v * (3.0 - 2.0 * v);
                    w = w * w * (3.0 - 2.0 * w);
                }

                let i = p.x().floor() as i32;
                let j = p.y().floor() as i32;
                let k = p.z().floor() as i32;
                let mut c = [[[0.0; 2]; 2]; 2];

                for di in 0..2 {
                    for dj in 0..2 {
                        for dk in 0..2 {
                            let index = self.perm_x[((i + di) & 255) as usize]
                                ^ self.perm_y[((j + dj) & 255) as usize]
                                ^ self.perm_z[((k + dk) & 255) as usize];
                            c[di as usize][dj as usize][dk as usize] +=
                                self.ranfloat[index as usize];
                        }
                    }
                }

                Self::trilinear_interp(c, u, v, w)
            }
        }
    }

    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * c[i][j][k];
                }
            }
        }

        accum
    }
}
