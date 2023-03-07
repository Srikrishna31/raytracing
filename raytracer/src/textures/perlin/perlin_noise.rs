use crate::{
    utils::{random_in_unit_interval, random_int},
    Point, Vec3,
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
pub(in crate::textures::perlin) trait Perlin {
    fn noise(&self, p: &Point) -> f64 {
        if self.use_turbulence() {
            self.turbulence(p)
        } else {
            self.actual_noise(p)
        }
    }

    fn actual_noise(&self, p: &Point) -> f64;

    fn use_turbulence(&self) -> bool;

    fn turbulence(&self, p: &Point) -> f64 {
        let mut accum = 0.0;
        let mut temp_p = *p;
        let mut weight = 1.0;
        const TURBULENCE_DEPTH: u32 = 7;

        for _ in 0..TURBULENCE_DEPTH {
            accum += weight * self.actual_noise(&temp_p);
            weight *= 0.5;
            temp_p *= 2.0;
        }

        accum.abs()
    }
}

struct PerlinCommon {
    perm_x: Vec<i32>,
    perm_y: Vec<i32>,
    perm_z: Vec<i32>,
    option: PerlinNoiseOptions,
    useturbulence: bool,
}

pub(in crate::textures::perlin) struct PerlinNoiseFloat {
    common: PerlinCommon,
    ranfloat: Vec<f64>,
}

pub(in crate::textures::perlin) struct PerlinNoiseVectors {
    common: PerlinCommon,
    ranvec: Vec<Vec3>,
}

impl Perlin for PerlinNoiseFloat {
    fn actual_noise(&self, p: &Point) -> f64 {
        match &self.common.option {
            PerlinNoiseOptions::Default => {
                let i = ((4.0 * p.x()) as i32 & 255) as usize;
                let j = ((4.0 * p.y()) as i32 & 255) as usize;
                let k = ((4.0 * p.z()) as i32 & 255) as usize;

                let index = self.common.perm_x[i] ^ self.common.perm_y[j] ^ self.common.perm_z[k];
                self.ranfloat[index as usize]
            }
            PerlinNoiseOptions::TrilinearSmoothing | PerlinNoiseOptions::HermitianSmoothing => {
                let mut u = p.x() - p.x().floor();
                let mut v = p.y() - p.y().floor();
                let mut w = p.z() - p.z().floor();

                if self.common.option == PerlinNoiseOptions::HermitianSmoothing {
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
                            let index = self.common.perm_x[((i + di) & 255) as usize]
                                ^ self.common.perm_y[((j + dj) & 255) as usize]
                                ^ self.common.perm_z[((k + dk) & 255) as usize];
                            c[di as usize][dj as usize][dk as usize] +=
                                self.ranfloat[index as usize];
                        }
                    }
                }

                Self::trilinear_interp(c, u, v, w)
            }
            _ => 0.0, // We will never hit this case
        }
    }

    fn use_turbulence(&self) -> bool {
        self.common.useturbulence
    }
}

impl Perlin for PerlinNoiseVectors {
    /// The output of perlin interpolation can return negative values. These negative values will be
    /// passed to the `sqrt()` function of our gamma function and get turned into `NaN`s. We will
    /// cast the perlin output back to between 0 and 1.
    fn actual_noise(&self, p: &Point) -> f64 {
        let u = p.x() - p.x().floor();
        let v = p.y() - p.y().floor();
        let w = p.z() - p.z().floor();
        let i = p.x().floor() as i32;
        let j = p.y().floor() as i32;
        let k = p.z().floor() as i32;
        let mut c = [[[Vec3::default(); 2]; 2]; 2];

        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    let index = self.common.perm_x[((i + di) & 255) as usize]
                        ^ self.common.perm_y[((j + dj) & 255) as usize]
                        ^ self.common.perm_z[((k + dk) & 255) as usize];
                    c[di as usize][dj as usize][dk as usize] += self.ranvec[index as usize];
                }
            }
        }

        let perlin_interp = Self::perlin_interp(c, u, v, w);
        if self.common.useturbulence {
            perlin_interp
        } else {
            0.5 * (1.0 + perlin_interp)
        }
    }

    fn use_turbulence(&self) -> bool {
        self.common.useturbulence
    }
}

impl PerlinNoiseVectors {
    fn perlin_interp(c: [[[Vec3; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let uu = u * u * (3.0 - 2.0 * u);
        let vv = v * v * (3.0 - 2.0 * v);
        let ww = w * w * (3.0 - 2.0 * w);
        let mut accum = 0.0;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    let ii = i as f64;
                    let jj = j as f64;
                    let kk = k as f64;

                    let weight = Vec3::new(u - ii, v - jj, w - kk);

                    accum += (ii * uu + (1.0 - ii) * (1.0 - uu))
                        * (jj * vv + (1.0 - jj) * (1.0 - vv))
                        * (kk * ww + (1.0 - kk) * (1.0 - ww))
                        * cijk.dot(&weight);
                }
            }
        }

        accum
    }

    pub fn new(opt: PerlinNoiseOptions, useturbulence: bool) -> PerlinNoiseVectors {
        let ranvec: Vec<Vec3> = (0..PerlinCommon::POINT_COUNT)
            .map(|_| Vec3::random_vector(-1.0, 1.0).unit_vector())
            .collect();

        PerlinNoiseVectors {
            ranvec,
            common: PerlinCommon::new(opt, useturbulence),
        }
    }
}

impl PerlinNoiseFloat {
    fn trilinear_interp(c: [[[f64; 2]; 2]; 2], u: f64, v: f64, w: f64) -> f64 {
        let mut accum = 0.0;

        for (i, ci) in c.iter().enumerate() {
            for (j, cij) in ci.iter().enumerate() {
                for (k, cijk) in cij.iter().enumerate() {
                    accum += (i as f64 * u + (1 - i) as f64 * (1.0 - u))
                        * (j as f64 * v + (1 - j) as f64 * (1.0 - v))
                        * (k as f64 * w + (1 - k) as f64 * (1.0 - w))
                        * cijk;
                }
            }
        }

        accum
    }

    pub fn new(opt: PerlinNoiseOptions, useturbulence: bool) -> PerlinNoiseFloat {
        let ranfloat: Vec<f64> = (0..PerlinCommon::POINT_COUNT)
            .map(|_| random_in_unit_interval())
            .collect();
        PerlinNoiseFloat {
            ranfloat,
            common: PerlinCommon::new(opt, useturbulence),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum PerlinNoiseOptions {
    Default,
    TrilinearSmoothing,
    HermitianSmoothing,
    VectorSmoothing,
}

impl PerlinCommon {
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

    pub fn new(option: PerlinNoiseOptions, useturbulence: bool) -> PerlinCommon {
        let perm_x = Self::perlin_generate_perm();
        let perm_y = Self::perlin_generate_perm();
        let perm_z = Self::perlin_generate_perm();

        PerlinCommon {
            perm_x,
            perm_y,
            perm_z,
            option,
            useturbulence,
        }
    }
}
