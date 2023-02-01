use crate::ray::Ray;
use crate::vec3::{Point, Vec3};
use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Point,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
}

#[derive(Debug, Clone)]
pub struct IntervalError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct IntersectionInterval {
    pub(crate) t_min: f64,
    pub(crate) t_max: f64,
}

impl Display for IntervalError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl Error for IntervalError {}

impl IntersectionInterval {
    pub fn new(t_min: f64, t_max: f64) -> Result<IntersectionInterval, IntervalError> {
        if t_min <= t_max {
            Ok(IntersectionInterval { t_min, t_max })
        } else {
            Err(IntervalError {
                message: "{t} should be between {t_min} and {t_max}".to_string(),
            })
        }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t: &IntersectionInterval) -> Option<HitRecord>;
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord {
            p: Vec3 { e: [0.0, 0.0, 0.0] },
            normal: Vec3 { e: [0.0, 0.0, 0.0] },
            t: 0.0,
            front_face: false,
        }
    }
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = r.direction().dot(outward_normal) < 0.0;
        self.normal = {
            if self.front_face {
                *outward_normal
            } else {
                -outward_normal
            }
        }
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self::new()
    }
}
