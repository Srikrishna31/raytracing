use crate::{Color, Point};

pub trait Texture {
    fn value(&self, u: f64, v: f64, p: &Point) -> Color;
}
