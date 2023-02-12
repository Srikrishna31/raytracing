use crate::rtweekend::{random, random_in_unit_interval};
use embed_doc_image::embed_doc_image;
use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub e: [f64; 3],
}

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Vec3 {
        Vec3 { e: [e0, e1, e2] }
    }

    pub fn length(&self) -> f64 {
        f64::sqrt(self.length_squared())
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn x(&self) -> f64 {
        self.e[0]
    }

    pub fn y(&self) -> f64 {
        self.e[1]
    }

    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }

    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(
            self.e[1] * rhs.e[2] - self.e[2] * rhs.e[1],
            self.e[2] * rhs.e[0] - self.e[0] * rhs.e[2],
            self.e[0] * rhs.e[1] - self.e[1] * rhs.e[0],
        )
    }

    pub fn unit_vector(&self) -> Vec3 {
        *self / self.length()
    }

    /// If the random unit vector generated is exactly opposite the normal vector, the two will
    /// sum to zero, which will result in a zero scatter direction vector. This leads to bad
    /// scenarios later on (infinities and NaNs), so we need to intercept the condition before we
    /// pass it on.
    pub fn near_zero(&self) -> bool {
        // Return true if the vector is close to zero in all dimensions.
        const S: f64 = 1e-8;
        f64::abs(self.e[0]) < S && f64::abs(self.e[1]) < S && f64::abs(self.e[2]) < S
    }

    pub fn random_unit_vector() -> Vec3 {
        Vec3 {
            e: [
                random_in_unit_interval(),
                random_in_unit_interval(),
                random_in_unit_interval(),
            ],
        }
    }

    pub fn random_vector(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [random(min, max), random(min, max), random(min, max)],
        }
    }

    /// # A Simple Diffuse Material
    /// Diffuse objects that don't emit light merely take on the color of their surroundings, but they
    /// modulate that with their own intrinsic color. Light that reflects off a diffuse surface has
    /// its direction randomized. So, if we send three rays into a crack between two diffuse surfaces
    /// they will each have different random behavior:
    ///
    /// ![Light ray bounces][raybounces]
    ///
    /// There are two unit radius spheres tangent to the hit point ***p*** of a surface. These two
    /// spheres have a center of **(P + n)** and **(P - n)**, where **n** is the normal of the
    /// surface. The sphere with a center at **(P - n)** is considered *inside* the surface, whereas
    /// the sphere with center **(P + n)** is considered *outside* the surface. Select the tangent
    /// unit radius sphere that is on the same side of the surface as the ray origin. Pick a random
    /// point **S** inside this unit radius sphere and send a ray from the hit point **P** to the
    /// random point **S**(this is the vector **(S - P)**):
    ///
    /// ![Generating a random diffuse bounce ray][randomdiffuseray]
    ///
    /// We need a way to pick a random point in a unit radius sphere. We'll use a rejection algorithm:
    /// First, pick a random point in the unit cube where x, y, and z all range from -1 to +1. Reject
    /// this point and try again if the point is outside the sphere.
    #[embed_doc_image("raybounces", "doc_images/light_ray_bounces.jpg")]
    #[embed_doc_image("randomdiffuseray", "doc_images/generating_a_random_diffuse_ray.jpg")]
    pub fn random_vector_in_unit_sphere() -> Vec3 {
        loop {
            let p = Self::random_vector(-1.0, 1.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }

    /// # True Lambertian Reflection
    /// The rejection method presented in `random_vector_in_unit_sphere` produces random points in
    /// the unit ball offset along the surface normal. This corresponds to picking directions on the
    /// hemisphere with high probability close to the normal, and a lower probability of scattering
    /// rays at grazing angles. This distribution scales by the **cos<sub>3</sub>(*φ*)** where *φ* is
    /// the angle from the normal. This is useful since light arriving at shallow angles spreads over
    /// a larger area, and thus has a lower contribution to the final color.
    ///
    /// However, we are interested in a Lambertian Distribution, which has a distribution of **cos(*φ*)**.
    /// True Lambertian has the probability higher for ray scattering close to the normal, but the
    /// distribution is more uniform. This is achieved by picking random points on the surface of
    /// the unit sphere, offset along the normal. Picking random points on the unit sphere can be
    /// achieved by picking random points *in* the unit sphere, and normalizing those.
    ///
    /// ![Generating a random unit vector][randvec]
    #[embed_doc_image("randvec", "doc_images/generating_a_random_unit_vector.png")]
    pub fn random_unit_vector_lambertian_distribution() -> Vec3 {
        Self::unit_vector(&Self::random_vector_in_unit_sphere())
    }

    pub fn random_unit_vector_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Self::random_vector_in_unit_sphere();
        if in_unit_sphere.dot(normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }

    pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
        *v - 2.0 * v.dot(n) * n
    }

    pub fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(-uv.dot(n), 1.0);
        let r_out_perpendicular = etai_over_etat * (*uv + cos_theta * n);
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perpendicular.length_squared())) * n;

        r_out_perpendicular + r_out_parallel
    }

    pub fn random_vector_in_unit_disk() -> Vec3 {
        loop {
            let p = Vec3::new(random(-1.0, 1.0), random(-1.0, 1.0), 0.0);
            if p.length_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;
    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl Neg for Vec3 {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Neg for &Vec3 {
    type Output = Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new(-self.e[0], -self.e[1], -self.e[2])
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() + rhs.x(), self.y() + rhs.y(), self.z() + rhs.z())
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() - rhs.x(), self.y() - rhs.y(), self.z() - rhs.z())
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x() * rhs.x(), self.y() * rhs.y(), self.z() * rhs.z())
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x() * rhs, self.y() * rhs, self.z() * rhs)
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        let t = 1.0 / rhs;
        self * t
    }
}

// For an expression like 5.0 * Vec3
impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

// For an expression accepting a Vector by reference
impl Mul<&Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: &Vec3) -> Self::Output {
        *rhs * self
    }
}

impl Default for Vec3 {
    fn default() -> Self {
        Vec3::new(0.0, 0.0, 0.0)
    }
}
