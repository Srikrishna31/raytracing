use once_cell::sync::Lazy;
use rand::prelude::*;

// As per the book's convention, this module will host all the constants needed.

pub const PI: f64 = std::f64::consts::PI;
pub const INFINITY: f64 = f64::INFINITY;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

/// Returns a random real in [0,1).
#[inline]
pub fn random_in_unit_interval() -> f64 {
    static mut RNG_THREAD: Lazy<ThreadRng> = Lazy::new(thread_rng);

    unsafe { RNG_THREAD.gen() }
}

/// Returns a random real in the range [min, max).
#[inline]
pub fn random(min: f64, max: f64) -> f64 {
    random_in_unit_interval() * (max - min) + min
}

#[inline]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    x
}
