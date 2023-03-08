use crate::materials::Material;
use crate::objects::HitRecord;
use crate::textures::{SolidColor, Texture};
use crate::{Color, Point, Ray};
use std::sync::Arc;

/// # Lights
/// Lighting is a key component of raytracing. Early simple raytracers used abstract light sources,
/// like points in space, or directions. Modern approaches have more physically based lights, which
/// have position and size. To create such light sources, we need to be able to take any regular
/// object and turn it into something that emits light into our scene.
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color {
        self.emit.value(u, v, p)
    }
}

pub enum Options {
    Texture(Arc<dyn Texture>),
    Clr(Color),
}

//TODO: Figure out a way to overload new to take an argument of texture or a color
impl DiffuseLight {
    pub fn new(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Arc::new(SolidColor::new(c)),
        }
    }

    pub fn new_with_texture(emit: Arc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }

    pub fn new_tex(opt: Options) -> DiffuseLight {
        match opt {
            Options::Texture(emit) => DiffuseLight { emit },
            Options::Clr(c) => DiffuseLight {
                emit: Arc::new(SolidColor::new(c)),
            },
        }
    }
}
