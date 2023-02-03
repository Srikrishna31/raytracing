use crate::HitRecord;
use crate::{Color, Ray};
use core::fmt::Debug;
use std::fmt::Formatter;

/// This trait represents the material which reflects the light falling on it, and also in a particular
/// color. If the material absorbs all light, then it is a black one, and if it reflects everything,
/// it would be a white one.
pub trait Material {
    /// Returns the reflected ray, and the color of the material. If the material is black, then it
    /// returns none.
    /// # Arguments
    /// `r_in`: Incoming / Incident ray
    ///
    /// `rec`: The object having this material property being hit.
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material trait: ")
    }
}
