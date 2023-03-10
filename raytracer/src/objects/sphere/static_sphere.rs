use super::common;
use crate::materials::{Dielectric, Material};
use crate::objects::{HitRecord, Hittable, AABB};
use crate::{Point, Ray, Vec3};
use embed_doc_image::embed_doc_image;
use std::sync::Arc;

#[derive(Clone)]
pub struct Sphere {
    center: Point,
    radius: f64,
    material: Arc<dyn Material>,
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
        common::hit(
            r,
            t_min,
            t_max,
            &|_| self.center,
            self.radius,
            &self.material,
        )
    }

    fn bounding_box(&self, _time0: f64, _time1: f64) -> Option<AABB> {
        let radius_dir = Vec3::new(self.radius, self.radius, self.radius);
        Some(AABB::new(
            self.center - radius_dir,
            self.center + radius_dir,
        ))
    }
}

impl Sphere {
    pub fn new(center: Point, radius: f64, material: Arc<dyn Material>) -> Sphere {
        Sphere {
            center,
            radius,
            material,
        }
    }
}

impl Default for Sphere {
    /// Returns a sphere centered at origin, with  radius 1 and with a glass material.
    fn default() -> Self {
        Sphere {
            center: Point::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Arc::new(Dielectric::new(1.5)),
        }
    }
}
