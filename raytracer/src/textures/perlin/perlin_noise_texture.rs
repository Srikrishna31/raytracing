use super::super::Texture;
use super::perlin_noise::{Perlin, PerlinNoiseFloat, PerlinNoiseOptions, PerlinNoiseVectors};
use crate::{Color, Point};
use once_cell::sync::Lazy;

pub struct PerlinNoiseTexture {
    noise: Box<dyn Perlin>,
    scale: f64,
}

impl Texture for PerlinNoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point) -> Color {
        if self.noise.use_turbulence() {
            *COLOR * 0.5 * (1.0 + (self.scale * p.z() + 10.0 * self.noise.noise(p)).sin())
        } else {
            *COLOR * self.noise.noise(&(self.scale * p))
        }
    }
}

// Due to lack of rust support to create constant structs, we have to resort to static constants.
static COLOR: Lazy<Color> = Lazy::new(|| Color::new(1.0, 1.0, 1.0));

impl PerlinNoiseTexture {
    pub fn new(opt: PerlinNoiseOptions, scale: f64, useturbulence: bool) -> PerlinNoiseTexture {
        let perlin: Box<dyn Perlin> = match opt {
            PerlinNoiseOptions::VectorSmoothing => {
                Box::new(PerlinNoiseVectors::new(opt, useturbulence))
            }
            _ => Box::new(PerlinNoiseFloat::new(opt, useturbulence)),
        };

        PerlinNoiseTexture {
            noise: perlin,
            scale,
        }
    }
}

impl Default for PerlinNoiseTexture {
    fn default() -> Self {
        PerlinNoiseTexture {
            noise: Box::new(PerlinNoiseFloat::new(PerlinNoiseOptions::Default, false)),
            scale: 1.0,
        }
    }
}
