use crate::Point;
use crate::utils::PI;

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
#[inline]
pub(in crate::objects::sphere) fn get_sphere_uv(p: &Point) -> (f64, f64) {
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
