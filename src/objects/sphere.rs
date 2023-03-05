use crate::materials::{Dielectric, Material};
use crate::objects::{HitRecord, Hittable, AABB};
use crate::utils::PI;
use crate::{Point, Ray, Vec3};
use embed_doc_image::embed_doc_image;
use std::rc::Rc;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    /// # Ray-Sphere Intersection
    /// The equation for a sphere centered at the origin of radius **R** is *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> = R<sup>2</sup>*
    /// Put another way, if a given point (x,y,z) is on the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> = R<sup>2</sup>*.
    /// If the given point is *inside* the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> < R<sup>2</sup>*,
    /// and if a given point is *outside* the sphere, then *x<sup>2</sup> + y<sup>2</sup> + z<sup>2</sup> >R<sup>2</sup>*. It gets
    /// uglier if the sphere center is at (C<sub>x</sub>, C<sub>y</sub>, C<sub>z</sub>):
    ///
    /// *(x - C<sub>x</sub>)<sup>2</sup> + (y - C<sub>y</sub>)<sup>2</sup> + (z - C<sub>z</sub>)<sup>2</sup> = r<sup>2</sup>*
    ///
    /// In graphics, you almost always want your formulas to be in terms of vectors so all the *x/y/z* stuff
    /// is under the hood in the `Vec3` class. The vector from center **C**=(C<sub>x</sub>,C<sub>y</sub>,C<sub>z</sub>)
    /// to point **P** = (*x,y,z*) is (**P** - **C**), and therefore
    ///
    /// (**P** - **C**) . (**P** - **C**) = (*x - C<sub>x</sub>*)<sup>2</sup> + (*y - C<sub>y</sub>*)<sup>2</sup> + (*z - C<sub>z</sub>*)<sup>2</sup>.
    ///
    /// So,the equation of the sphere in the vector form is:
    ///
    /// (**P - C**).(**P - C**) = r<sup>2</sup>
    ///
    /// We can read this as "any point **P** that satisfies this equation is on the sphere". We want to
    /// know if our ray **P**(*t*) = **A** + *t***b** ever hits the sphere anywhere. If it does hit the
    /// sphere, there is some *t* for which **P**(*t*) satisfies the sphere equation. So we are looking
    /// for any *t* where this is true:
    ///
    /// (**P**(*t*) - **C**).(**P**(*t*) - **C**) = r<sup>2</sup>
    ///
    /// or expanding the full form of the ray **P**(*t*):
    ///
    /// (**A** + *t***b** - **C**).(**A** + *t***b** - **C**) = *r*<sup>2</sup>
    ///
    /// The rules of vector algebra are all that we want here. If we expand that equation and move all
    /// the terms to the left hand side we get:
    ///
    /// *t<sup>2</sup>***b**.**b** + 2*t***b**.(**A - C**) + (**A - C**).(**A - C**) - r<sup>2</sup> = 0
    ///
    /// The vectors and *r* in that equation are all constant and known. The unknown is t, and the equation
    /// is a quadratic. You can solve for *t* and there is a square root part that is either positive
    /// (meaning two real solutions), negative (meaning no real solutions), or zero (meaning one real
    /// solution).
    ///
    /// ![Ray-Sphere intersection results][raysphere]
    #[embed_doc_image("raysphere", "doc_images/ray_sphere_intersection_results.jpg")]
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = r.origin() - self.center;
        let a = r.direction().length_squared();
        let half_b = oc.dot(&r.direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || root > t_max {
            root = (-half_b + sqrtd) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let p = r.at(root);
        let outward_normal = (p - self.center) / self.radius;
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        let mut hit_rec = HitRecord {
            t: root,
            p,
            normal: Vec3::new(0.0, 0.0, 0.0),
            front_face: false,
            mat: self.material.clone(),
            u,
            v,
        };
        hit_rec.set_face_normal(r, &outward_normal);

        Some(hit_rec)
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        Some(AABB::new(
            self.center - Vec3::new(self.radius, self.radius, self.radius),
            self.center + Vec3::new(self.radius, self.radius, self.radius),
        ))
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Rc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }

    /// # Texture Coordinates for Spheres
    /// For spheres, texture coordinates are usually based on some form of longitude and latitude, i.e.,
    /// spherical coordinates. So we compute **(θ, φ)** in spherical coordinates, where **θ** is the
    /// angle up from the bottom pole (that is, up from -Y), and **φ** is the angle around the Y-axis
    /// (from -X to +Z to +X to -Z back to -X).
    ///
    /// We want to map **θ** and **φ** to texture coordinates *u* and *v* each in **[0,1]**, where
    /// (*u*=0, *v*=0) maps to the bottom-left corner of the texture. Thus the normalization from
    /// **(θ, φ)** to **(u,v)** would be:
    ///
    /// ```math
    ///     u = \frac{φ}{2π}
    ///     v = \frac{θ}{2π}
    /// ```
    ///
    /// To compute *θ* and *φ* for a given point on the unit sphere centered at the origin, we start
    /// with the equations for the corresponding Cartesian coordinates:
    ///
    /// ```math
    ///     y = -cos(θ)
    ///     x = -cos(φ)sin(θ)
    ///     z = sin(φ)sin(θ)
    /// ```
    ///
    /// We need to invert these equations to solve for **θ** and **φ**. Using the `atan2` or the tan<sup>-1</sup>
    /// function, we can pass in x and z(the **sin(θ)**) cancel) to solve for **φ**:
    ///
    /// ```math
    ///     φ = tan^-1(\frac{z}{-x})
    /// ```
    ///
    /// `atan2()` returns values in the range -π to π, but they go from 0 to π, then flip to -π and
    /// proceed back to 0. While this is mathematically correct, we want *u* to range from 0 to 1,
    /// not from **0 to ½** and then from **-½ to 0**. Fortunately,
    ///
    /// ```math
    ///     atan2(a,b) = atan2(-a, -b) + π,
    /// ```
    /// and the second forumulation yields values from 0 continuously to 2π. Thus, we can compute φ as
    ///
    /// ```math
    ///     φ = atan2(-z,x) + π
    /// ```
    ///
    /// The derivation for **θ** is more straightforward:
    ///
    /// ```math
    ///     θ = cos^-1(-y)
    /// ```
    fn get_sphere_uv(p: &Point) -> (f64, f64) {
        // p: a given point on the sphere of radius one, centered at the origin.
        // u: returned value [0,1] of angle around the Y axis from X=-1.
        // v: returned value [0,1] of angle from Y=-1 to Y=+1.
        //      <1 0 0> yields <0.50 0.50>      <-1 0 0> yields <0.00 0.50>
        //      <0 1 0> yields <0.50 1.00>      <0 -1 0> yields <0.50 0.00>
        //      <0 0 1> yields <0.25 0.50>      <0  0 -1> yields <0.75, 0.50>
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        (phi / (2.0 * PI), theta / PI)
    }
}

impl Default for Sphere {
    /// Returns a sphere centered at origin, with  radius 1 and with a glass material.
    fn default() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Rc::new(Dielectric::new(1.5)),
        }
    }
}
