use crate::HitRecord;
use crate::{Color, Ray};
use core::fmt::Debug;
use std::fmt::Formatter;

pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

impl Debug for dyn Material {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Material trait: ")
    }
}
