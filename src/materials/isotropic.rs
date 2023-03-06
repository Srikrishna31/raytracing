use crate::materials::Material;
use crate::objects::HitRecord;
use crate::textures::{SolidColor, Texture};
use crate::Vec3;
use crate::{Color, Ray};
use std::sync::Arc;

pub struct Isotropic {
    albedo: Arc<dyn Texture>,
}

impl Material for Isotropic {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let scattered_ray =
            Ray::new_with_time(&rec.p, &Vec3::random_vector_in_unit_sphere(), r_in.time());
        let attenuation = self.albedo.value(rec.u, rec.v, &rec.p);

        Some((scattered_ray, attenuation))
    }
}

impl Isotropic {
    pub fn new(a: Arc<dyn Texture>) -> Isotropic {
        Isotropic { albedo: a }
    }

    pub fn new_with_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: Arc::new(SolidColor::new(c)),
        }
    }
}
