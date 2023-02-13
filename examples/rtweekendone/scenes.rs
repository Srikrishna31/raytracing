use raytracing::materials::{Dielectric, LambertianMaterial, Metal};
use raytracing::objects::{MovingSphere, Sphere, World};
use raytracing::utils::{random, random_in_unit_interval, PI};
use raytracing::{Camera, Color, ImageSettings, Point, Scene, Vec3};

use raytracing::textures::{
    CheckerTexture, ImageTexture, PerlinNoiseOptions, PerlinNoiseTexture, SolidColor,
};
use std::path::Path;
use std::rc::Rc;

pub(crate) fn scene_with_dielectric_and_shiny_sphere() -> World {
    let mut world = World::new();
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
pub fn scene_with_hollow_glass_sphere() -> World {
    let mut world = World::new();
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

pub fn scene_for_wide_angle_camera() -> Scene {
    let R = (PI / 4.0).cos();
    let mut world = World::new();

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
        0.0,
        0.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn scene_with_alternate_viewpoint() -> Scene {
    let mut world = World::new();

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
        0.0,
        0.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn scene_with_depth_of_field_camera() -> Scene {
    let mut world = World::new();

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
        0.0,
        0.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn rtweekend_one_final_scene(settings: &ImageSettings) -> Scene {
    let mut world = World::new();

    let ground_material = Rc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_in_unit_interval();
            let center = Point::new(
                a as f64 + 0.9 * random_in_unit_interval(),
                0.2,
                b as f64 + 0.9 * random_in_unit_interval(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit_vector() * Color::random_unit_vector();
                    let sphere_material = Rc::new(LambertianMaterial::new(albedo));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;
    let aspect_ratio = settings.aspect_ratio;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        0.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

/// The code below takes the example diffuse spheres from the scene at the end of the last book (Ray
/// tracing in a weekend), and makes them move during the image render. (Think of a camera with
/// shutter opening at time 0 and closing at time 1.) Each sphere moves from its center **C** at time
/// ***t*** **= 0** to **C + (0, r/2, 0)** at time ***t*** **=1**, where *r* is a random number in
/// **[0,1)**:
pub fn rtweekend_one_final_scene_with_moving_spheres(settings: &ImageSettings) -> Scene {
    let mut world = World::new();

    let ground_material = Rc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_in_unit_interval();
            let center = Point::new(
                a as f64 + 0.9 * random_in_unit_interval(),
                0.2,
                b as f64 + 0.9 * random_in_unit_interval(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit_vector() * Color::random_unit_vector();
                    let sphere_material = Rc::new(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random(0.0, 0.5), 0.0);
                    world.add(Rc::new(
                        MovingSphere::new(center, center2, 0.2, sphere_material, 0.0, 1.0).unwrap(),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn rtweekend_one_final_scene_with_moving_spheres_checkered_texture(
    settings: &ImageSettings,
) -> Scene {
    let mut world = World::new();

    let checker = Rc::new(CheckerTexture::new(
        Rc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let ground_material = Rc::new(LambertianMaterial::new_with_texture(checker));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = random_in_unit_interval();
            let center = Point::new(
                a as f64 + 0.9 * random_in_unit_interval(),
                0.2,
                b as f64 + 0.9 * random_in_unit_interval(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // diffuse
                    let albedo = Color::random_unit_vector() * Color::random_unit_vector();
                    let sphere_material = Rc::new(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random(0.0, 0.5), 0.0);
                    world.add(Rc::new(
                        MovingSphere::new(center, center2, 0.2, sphere_material, 0.0, 1.0).unwrap(),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn two_checkered_spheres(settings: &ImageSettings) -> Scene {
    let checker = Rc::new(CheckerTexture::new(
        Rc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Rc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let mut world = World::new();

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(LambertianMaterial::new_with_texture(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(LambertianMaterial::new_with_texture(checker)),
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        40.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn perlin_textured_spheres(settings: &ImageSettings) -> Scene {
    let pertext = Rc::new(PerlinNoiseTexture::default());

    let mut world = World::new();

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext)),
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn perlin_smoothed_textured_spheres(settings: &ImageSettings) -> Scene {
    let pertext = Rc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::HermitianSmoothing,
        4.0,
        false,
    ));

    let mut world = World::new();

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext)),
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

pub fn marble_spheres(settings: &ImageSettings) -> Scene {
    let pertext = Rc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::VectorSmoothing,
        4.0,
        true,
    ));

    let mut world = World::new();

    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(LambertianMaterial::new_with_texture(pertext)),
    )));

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}

fn earth() -> World {
    let path = std::env::current_dir()
        .unwrap()
        .join(Path::new("./examples/rtweekendone/earthmap.jpg"));

    let earth_texture = Rc::new(ImageTexture::new(&path));
    let earth_surface = Rc::new(LambertianMaterial::new_with_texture(earth_texture));
    let globe = Rc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    World::new_with_object(globe)
}

pub fn earth_scene(settings: &ImageSettings) -> Scene {
    let world = earth();

    let lookfrom = Point::new(13.0, 2.0, 3.0);
    let lookat = Point::new(0.0, 0.0, 0.0);
    let vup = Point::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;
    let aperture = 0.0;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        settings.aspect_ratio,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    Scene::new(world, camera, Color::new(0.7, 0.8, 1.0))
}
