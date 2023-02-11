use crate::textures::Texture;
use crate::{Color, Point};

/// A texture in graphics usually means a function that makes the colors on a surface procedural.
/// This procedure can be synthesis code, or it could be an image lookup, or a combination of both.
/// But we will model constant colors also as textures for simplicity and elegance.
pub struct SolidColor {
    color: Color,
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point) -> Color {
        self.color
    }
}

impl SolidColor {
    pub fn new(color: Color) -> SolidColor {
        SolidColor { color }
    }
}

impl Default for SolidColor {
    /// Returns a texture producing white color.
    fn default() -> Self {
        SolidColor {
            color: Color::new(1.0, 1.0, 1.0),
        }
    }
}
