use crate::{Color, HitRecord, Material, Ray, Vec3};
use embed_doc_image::embed_doc_image;

/// # Dielectrics
/// Clear materials such as water, glass, and diamonds are dielectrics. When a light ray hits them,
/// it splits into a reflected ray and a refracted(transmitted) ray. We'll handle that by randomly
/// choosing between reflection or refraction, and only generating one scattered ray per interaction.
///
/// ## Refraction and Snell's Law
/// The refraction is described by Snell's law:
///                     **η.sinθ = η'.sinθ'**
/// where **θ** and **θ'** are the angles from the normal, and **η** and **η'** (pronounced "eta" and
/// "eta prime") are the refractive indices (typically air = 1.0, glass = 1.3-1.7, diamond=2.4). The
/// geometry is:
///
/// ![Ray Refraction][rayrefract]
///
/// In order to determine the direction of the refracted ray, we have to solve for **sinθ'**:
/// ```math
///     sinθ' = \frac{η}{η'} . sinθ
/// ```
#[embed_doc_image("rayrefract", "doc_images/ray_refraction.jpg")]
#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    ir: f64, // Index of Refraction
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r_in.direction().unit_vector();
        let refracted = Vec3::refract(&unit_direction, &rec.normal, refraction_ratio);
        let scattered = Ray::new(&rec.p, &refracted);

        Some((scattered, attenuation))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}
