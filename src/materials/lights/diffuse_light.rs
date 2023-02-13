use crate::materials::Material;
use crate::objects::HitRecord;
use crate::textures::{SolidColor, Texture};
use crate::{Color, Point, Ray};
use std::rc::Rc;

/// # Lights
/// Lighting is a key component of raytracing. Early simple raytracers used abstract light sources,
/// like points in space, or directions. Modern approaches have more physically based lights, which
/// have position and size. To create such light sources, we need to be able to take any regular
/// object and turn it into something that emits light into our scene.
pub struct DiffuseLight {
    emit: Rc<dyn Texture>,
}

impl Material for DiffuseLight {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        None
    }

    fn emitted(&self, u: f64, v: f64, p: &Point) -> Color {
        self.emit.value(u, v, p)
    }
}

//TODO: Figure out a way to overload new to take an argument of texture or a color
impl DiffuseLight {
    pub fn new(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: Rc::new(SolidColor::new(c)),
        }
    }

    pub fn new_with_texture(emit: Rc<dyn Texture>) -> DiffuseLight {
        DiffuseLight { emit }
    }
}
