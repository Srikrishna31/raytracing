use super::material::Material;
use crate::objects::HitRecord;
use crate::{Color, Ray, Vec3};
use embed_doc_image::embed_doc_image;

/// # Mirrored Light Reflection
/// For smooth metals, the ray won't be randomly scattered.
///
/// ![Ray Reflection][rayreflect]
///
/// The reflected ray direction in red is just **v + 2b**. In our design, **n** is a unit vector, but
/// **v** may not be. The length of **b** should be **v.n**. Because **v** points in, we will need
/// a minus sign yielding:
///
/// **v - (2v.n)xn**.
///
/// # Fuzzy Reflection
/// We can also randomize the reflected direction by using a small sphere and choosing a new endpoint
/// for the ray:
///
/// ![Generating fuzzed reflection rays][fuzzreflect]
///
/// The bigger the sphere, the fuzzier the reflections will be. This suggests adding a fuzziness
/// parameter that is just the radius of the sphere (so zero is no perturbation). The catch is that
/// for big spheres or grazing rays, we may scatter below the surface. We can just have the surface
/// absorb those.
#[embed_doc_image("rayreflect", "doc_images/ray_reflection.jpg")]
#[embed_doc_image("fuzzreflect", "doc_images/generating_fuzzed_reflection_rays.jpg")]
#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let reflected_ray = Vec3::reflect(&r_in.direction().unit_vector(), &rec.normal);
        let scattered = Ray::new(
            &rec.p,
            &(reflected_ray + self.fuzz * Vec3::random_vector_in_unit_sphere()),
        );
        if scattered.direction().dot(&rec.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

impl Metal {
    pub fn new(a: Color, f: f64) -> Metal {
        let f_ = if f < 1.0 { f } else { 1.0 };

        Metal {
            albedo: a,
            fuzz: f_,
        }
    }
}
