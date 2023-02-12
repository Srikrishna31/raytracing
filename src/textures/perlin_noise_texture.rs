use super::perlin::{Perlin, PerlinNoiseOptions};
use crate::textures::Texture;
use crate::{Color, Point};
use once_cell::sync::Lazy;

pub struct PerlinNoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        *COLOR * self.noise.noise(&(self.scale * p))
    }
}

// Due to lack of rust support to create constant structs, we have to resort to static constants.
static COLOR: Lazy<Color> = Lazy::new(|| Color::new(1.0, 1.0, 1.0));

impl PerlinNoiseTexture {
    pub fn new(opt: PerlinNoiseOptions, scale: f64) -> PerlinNoiseTexture {
        PerlinNoiseTexture {
            noise: Perlin::new(opt),
            scale,
        }
    }
}

impl Default for PerlinNoiseTexture {
    fn default() -> Self {
        PerlinNoiseTexture {
            noise: Perlin::new(PerlinNoiseOptions::Default),
            scale: 1.0,
        }
    }
}
