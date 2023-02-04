use raytracing::{
    Camera, Color, Dielectric, HittableList, LambertianMaterial, Metal, Point, Sphere, Vec3, PI,
};
use std::rc::Rc;

pub(crate) fn scene_with_dielectric_and_shiny_sphere() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    world
}

/// An interesting and easy trick with dielectric spheres is to note that if you use a negative
/// radius, the geometry is unaffected, but the surface normal points inward. This can be used as α
/// bubble to make a hollow glass sphere:
pub fn scene_with_hollow_glass_sphere() -> HittableList {
    let mut world = HittableList::new();
    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));

    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    world
}

pub fn scene_for_wide_angle_camera() -> (HittableList, Camera) {
    let R = (PI / 4.0).cos();
    let mut world = HittableList::new();

    let material_left = Rc::new(LambertianMaterial::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(LambertianMaterial::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Rc::new(Sphere::new(
        Point::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(R, 0.0, -1.0),
        R,
        material_right,
    )));

    let camera = Camera::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(0.0, 0.0, -1.0),
        Point::new(0.0, 1.0, 0.0),
        90.0,
        16.0 / 9.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn scene_with_alternate_viewpoint() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    //Camera with far viewing
    //let camera = Camera::new(Point::new(-2.0,2.0,1.0), Point::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, 16.0/9.0, 0.0, 1.0);

    //Camera with zoom in view
    let camera = Camera::new(
        Point::new(-2.0, 2.0, 1.0),
        Point::new(0.0, 0.0, -1.0),
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.0,
        1.0,
    );

    (world, camera)
}

pub fn scene_with_depth_of_field_camera() -> (HittableList, Camera) {
    let mut world = HittableList::new();

    let material_ground = Rc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.5));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    //Camera with far viewing
    //let camera = Camera::new(Point::new(-2.0,2.0,1.0), Point::new(0.0, 0.0, -1.0), Vec3::new(0.0, 1.0, 0.0), 90.0, 16.0/9.0, 0.0, 1.0);

    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = (lookfrom - lookat).length();
    let aperture = 2.0;
    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        16.0 / 9.0,
        aperture,
        dist_to_focus,
    );

    (world, camera)
}
