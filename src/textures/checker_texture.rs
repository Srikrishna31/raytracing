use super::Texture;
use crate::textures::SolidColor;
use crate::{Color, Point};
use std::rc::Rc;

/// We can create a checker texture by noting that the sign of sine and cosine just alternates in α
/// regular way, and if we multiply trig functions in all three dimensions, the sign of that product
/// forms a 3D checker pattern.
pub struct CheckerTexture {
    odd: Rc<dyn Texture>,
    even: Rc<dyn Texture>,
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color {
        let sines = (10.0 * p.x()).sin() * (10.0 * p.y()).sin() * (10.0 * p.z()).sin();

        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}

impl CheckerTexture {
    pub fn new(odd: Rc<dyn Texture>, even: Rc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
}

impl Default for CheckerTexture {
    /// Returns a checker pattern of white and black colors.
    fn default() -> Self {
        CheckerTexture::new(
            Rc::new(SolidColor::new(Color::new(0.0, 0.0, 0.0))),
            Rc::new(SolidColor::new(Color::new(1.0, 1.0, 1.0))),
        )
    }
}
