// As per the book's convention, this module will host all the constants needed.

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
