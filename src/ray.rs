use crate::vec3::{Point, Vec3};
use std::clone::Clone;

/// The one thing that all ray tracers have is a ray class and a computation of what color is seen
/// along a ray. We can think of a ray as a function: **P**(*t*) = **A** + *t***b**.
///
/// Here **P** is a 3D position along a line in 3D. **A** is the ray origin and **b** is the ray
/// direction. The ray parameter *t* is a real number (`f64` in code). Plug in a different *t* and
/// **P**(*t*) moves the point along the ray. Add in negative *t* values and you can go anywhere on
/// the 3D line. For positive *t*, you get only the parts in front of **A**, and this is what is
/// often called a half-line or ray.a
pub struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: &Point, direction: &Vec3) -> Ray {
        Ray {
            origin: origin.clone(),
            direction: direction.clone(),
        }
    }

    pub fn at(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn direction(&self) -> Vec3 {
        self.direction
    }

    pub fn origin(&self) -> Point {
        self.origin
    }
}