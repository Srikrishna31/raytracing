use crate::materials::Material;
use crate::objects::HitRecord;
use crate::textures::{SolidColor, Texture};
use crate::{Color, Ray};
use crate::Vec3;
use std::rc::Rc;

pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered_ray = Ray::new_with_time(&rec.p, &Vec3::random_vector_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((scattered_ray, attenuation))
    }
}

impl Isotropic {
    pub fn new(a: Rc<dyn Texture>) -> Isotropic {
        Isotropic { albedo: a }
    }

    pub fn new_with_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: Rc::new(SolidColor::new(c)),
        }
    }
}
