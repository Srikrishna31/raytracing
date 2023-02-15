use crate::materials::Material;
use crate::objects::{HitRecord, Hittable, AABB};
use crate::utils::{random_in_unit_interval, INFINITY};
use crate::{Ray, Vec3};
use embed_doc_image::embed_doc_image;
use std::rc::Rc;

/// # Volumes
/// It's nice to add smoke/fog/mist to a raytracer. These are sometimes called *volumes* or
/// *participating media*. Another feature that is nice to add is subsurface scattering, which is
/// sort of like dense fog inside an object. This usually adds software architectural mayhem because
/// volumes are a different animal than surfaces, but a cute technique is to make a volume a random
/// surface. A bunch of smoke can be replaced with a surface that probabilistically might or might
/// not be there at every point in the volume.
///
/// ## Constant Density Mediums
/// First, let's start with a volume of constant density. A ray going through there can either scatter
/// inside the volume, or it can make it all the way through like the middle ray in the figure. More
/// thin transparent volumes, like a light fog, are more likely to have rays like the middle one.
/// How far the ray has to travel through the volume also determines how likely it is for the ray to
/// make it through.
///
/// !["Ray-volume intersection"][rayvolume]
///
/// As the ray passes through the volume, it may scatter at any point. The denser the volume, the
/// more likely that is. The probability that the ray scatters in any small distance δL is:
///
/// ```math
///     probability = C.δL
/// ```
///
/// where ***C*** is proportinal to the optical density of the volume. For a random number you get
/// a distance where the scattering occurs. If that distance is outside the volume, then there no "hit".
/// For a constant volume we just need the density ***C*** and the boundary.
#[embed_doc_image("rayvolume", "doc_images/ray_volume_interaction.jpg")]
pub struct ConstantMedium {
    neg_inv_density: f64,
    boundary: Rc<dyn Hittable>,
    phase_function: Rc<dyn Material>,
}

impl Hittable for ConstantMedium {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Print occasional samples when debugging. To enable, set enableDebug true.
        const ENABLE_DEBUG: bool = false;
        let debugging = ENABLE_DEBUG && random_in_unit_interval() < 0.00001;

        let (rec1, rec2) = self.boundary.hit(r, -INFINITY, INFINITY).map_or_else(
            || (None, None),
            |r1| {
                self.boundary
                    .hit(r, r1.t + 0.0001, INFINITY)
                    .map_or_else(|| (None, None), |r2| (Some(r1), Some(r2)))
            },
        );

        match (rec1, rec2) {
            (Some(mut r1), Some(mut r2)) => {
                if debugging {
                    eprintln!("t_min={}, t_max={}", &r1.t, &r2.t);
                }

                if r1.t < t_min {
                    r1.t = t_min;
                }
                if r2.t > t_max {
                    r2.t = t_max;
                }

                if r1.t >= r2.t {
                    return None;
                }

                if r1.t < 0.0 {
                    r1.t = 0.0;
                }

                let ray_length = r.direction().length();
                let distance_inside_boundary = (r2.t - r1.t) * ray_length;
                let hit_distance = self.neg_inv_density * random_in_unit_interval().log(10.0); // Todo: CHeck if this needs to change to ln

                if hit_distance > distance_inside_boundary {
                    return None;
                }

                let t = r1.t + hit_distance * ray_length;
                let p = r.at(t);

                if debugging {
                    eprintln!(
                        "hit_distance = {}\n, rec.t = {}\n, rec.p = {:?}\n",
                        hit_distance, &t, &p
                    );
                }

                Some(HitRecord::new_with_all_params(
                    p,
                    Vec3::new(1.0, 0.0, 0.0),
                    self.phase_function.clone(),
                    t,
                    0.0,
                    0.0,
                    true,
                ))
            }
            _ => None,
        }
    }

    fn bounding_box(&self, time0: f64, time1: f64) -> Option<AABB> {
        self.boundary.bounding_box(time0, time1)
    }
}

impl ConstantMedium {
    pub fn new(
        boundary: Rc<dyn Hittable>,
        phase_function: Rc<dyn Material>,
        density: f64,
    ) -> ConstantMedium {
        ConstantMedium {
            boundary,
            phase_function,
            neg_inv_density: 1.0 / density,
        }
    }
}
