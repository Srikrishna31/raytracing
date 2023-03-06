use crate::materials::material;
use crate::objects::HitRecord;
use crate::textures::{SolidColor, Texture};
use crate::{Color, Ray, Vec3};
use material::Material;
use std::sync::Arc;

/// The Lambertian material, it can either scatter always and attenuate by its reflectance **R**, or
/// it can scatter with no attenuation but absorb the fraction **1 - R** of the rays, or it could be
/// a mixture of those strategies.
#[derive(Clone)]
pub struct LambertianMaterial {
    albedo: Arc<dyn Texture>,
}

impl Material for LambertianMaterial {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        Some((
            Ray::new_with_time(&rec.p, &scatter_direction, r_in.time()),
            self.albedo.value(rec.u, rec.v, &rec.p),
        ))
    }
}

impl LambertianMaterial {
    pub fn new(a: Color) -> LambertianMaterial {
        LambertianMaterial {
            albedo: Arc::new(SolidColor::new(a)),
        }
    }

    pub fn new_with_texture(a: Arc<dyn Texture>) -> LambertianMaterial {
        LambertianMaterial { albedo: a }
    }
}
