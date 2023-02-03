use crate::{Color, HitRecord, Material, Ray, Vec3};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_ray = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(&rec.p, &reflected_ray);

        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(a: Color) -> Metal {
        Metal { albedo: a }
    }
}
