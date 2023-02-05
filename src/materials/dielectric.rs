use crate::utils::random_in_unit_interval;
use crate::{materials::Material, objects::HitRecord};
use crate::{Color, Ray, Vec3};
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
///
/// On the refracted side of the surface there is a refracted ray **R'** and a normal **n'**, and
/// there exists an angle, **θ'**, between them. We can split **R'** into the parts of the ray that
/// are perpendicular to **n'** and parallel to **n'**:
///
/// ```math
///     R' = R⟂ + R∥
/// ```
///
/// If we solve for $R'\perp$ and $R'\parallel$ we get:
///
/// ```math
///     R'⟂ = \frac{η}{η'}(R + cosθn)
///
///     R'∥ = -√(1- |R'⟂|²)
///
/// We still need to solve for **cosθ**. It is well known that the dot product of two vectors can be
/// explained in terms of the cosine of the angle between them:
///
/// ```math
///     a.b = |a||b|cosθ
/// ```
///
/// We can now rewrite **R'**⟂ in terms of known quantities:
///
/// ```math
///     R'⟂ = \frac{η}{η'}(R + (-R.n)n)
/// ```
///
/// When we combine them back together, we can write a function to calculate **R'**
///
/// # Total Internal Reflection
///
/// One troublesome practical issue is that when the ray is in the material with the higher refractive
/// index, there is no real solution to Snell's law, and thus there is no refraction possible. If we
/// refer back to Snell's law and the derivation of sinθ':
///
/// ```math
///     sinθ' = \frac{η}{η'}.sinθ
/// ```
///
/// If the ray is inside glass and outside is air (η=1.5 and η'=1.0):
///
/// ```math
///     sinθ' = \frac{1.5}{1.0}.sinθ
/// ```
///
/// The value of **sinθ'** cannot be greater than 1. So, if,
///
/// ```math
///     \frac{1.5}{1.0}.sinθ > 1.0
/// ```
/// , the equality between the two sides of the equation is broken, and a solution cannot exist. If α
/// solution doesnot exist, **the glass cannot refract, and therefore must reflect the ray**.
///
/// Here all the light is reflected, and because in practice that is usually inside solid objects, it
/// is called "total internal reflection". This is why sometimes the water-air boundary acts as a
/// perfect mirror when you are submerged.
///
/// We can solve for `sin_theta` using the trigonometric qualities:
///
/// ```math
///     sinθ = √(1 - cos²θ)
///     cosθ = R.n
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
        let cos_theta = f64::min(-unit_direction.dot(&rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || reflectance(cos_theta, refraction_ratio) > random_in_unit_interval()
        {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
            Vec3::refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        let scattered = Ray::new(&rec.p, &direction);

        Some((scattered, attenuation))
    }
}

impl Dielectric {
    pub fn new(ir: f64) -> Dielectric {
        Dielectric { ir }
    }
}

fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
    // Use Schlick's approximation for reflectance.
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;

    r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
}
