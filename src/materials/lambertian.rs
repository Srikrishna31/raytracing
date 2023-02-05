use crate::materials::material;
use crate::objects::HitRecord;
use crate::{Color, Ray, Vec3};
use material::Material;

/// The Lambertian material, it can either scatter always and attenuate by its reflectance **R**, or
/// it can scatter with no attenuation but absorb the fraction **1 - R** of the rays, or it could be
/// a mixture of those strategies.
#[derive(Debug, Clone, Copy)]
pub struct LambertianMaterial {
    albedo: Color,
}

impl Material for LambertianMaterial {
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((Ray::new(&rec.p, &scatter_direction), self.albedo))
    }
}

impl LambertianMaterial {
    pub fn new(a: Color) -> LambertianMaterial {
        LambertianMaterial { albedo: a }
    }
}
