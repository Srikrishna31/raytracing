use raytracer::materials::{Dielectric, LambertianMaterial, Metal};
use raytracer::objects::{
    Hittable, MovingSphere, RotateY, Sphere, Translate, World, XYRect, XZRect, YZRect,
};
use raytracer::utils::{random, random_in_unit_interval, PI};
use raytracer::{objects, Camera, Color, ImageSettings, Point, Scene, Vec3};

use raytracer::materials::lights::DiffuseLight;
use raytracer::objects::volumes::ConstantMedium;
use raytracer::textures::{
    CheckerTexture, ImageTexture, PerlinNoiseOptions, PerlinNoiseTexture, SolidColor, Texture,
};
use std::path::Path;
use std::sync::Arc;

pub(crate) fn scene_with_dielectric_and_shiny_sphere() -> World {
    let mut world = World::new();
    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
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
    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.4,
        material_left,
    )));

    world.add(Arc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    world
}

pub fn scene_for_wide_angle_camera() -> Scene {
    let R = (PI / 4.0).cos();
    let mut world = World::new();

    let material_left = Arc::new(LambertianMaterial::new(Color::new(0.0, 0.0, 1.0)));
    let material_right = Arc::new(LambertianMaterial::new(Color::new(1.0, 0.0, 0.0)));

    world.add(Arc::new(Sphere::new(
        Point::new(-R, 0.0, -1.0),
        R,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
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

    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
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

    let material_ground = Arc::new(LambertianMaterial::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(LambertianMaterial::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
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

    let ground_material = Arc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
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
                    let sphere_material = Arc::new(LambertianMaterial::new(albedo));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
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

    let ground_material = Arc::new(LambertianMaterial::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
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
                    let sphere_material = Arc::new(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random(0.0, 0.5), 0.0);
                    world.add(Arc::new(
                        MovingSphere::new(center, center2, 0.2, sphere_material, 0.0, 1.0).unwrap(),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
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

    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));
    let ground_material = Arc::new(LambertianMaterial::new_with_texture(checker));
    world.add(Arc::new(Sphere::new(
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
                    let sphere_material = Arc::new(LambertianMaterial::new(albedo));
                    let center2 = center + Vec3::new(0.0, random(0.0, 0.5), 0.0);
                    world.add(Arc::new(
                        MovingSphere::new(center, center2, 0.2, sphere_material, 0.0, 1.0).unwrap(),
                    ));
                } else if choose_material < 0.95 {
                    // metal
                    let albedo = Color::random_vector(0.5, 1.0);
                    let fuzz = random(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(LambertianMaterial::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
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
    let checker = Arc::new(CheckerTexture::new(
        Arc::new(SolidColor::new(Color::new(0.2, 0.3, 0.1))),
        Arc::new(SolidColor::new(Color::new(0.9, 0.9, 0.9))),
    ));

    let mut world = World::new();

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(LambertianMaterial::new_with_texture(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(LambertianMaterial::new_with_texture(checker)),
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
    let pertext = Arc::new(PerlinNoiseTexture::default());

    let mut world = World::new();

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext)),
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
    let pertext = Arc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::HermitianSmoothing,
        4.0,
        false,
    ));

    let mut world = World::new();

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext)),
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
    let pertext = Arc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::VectorSmoothing,
        4.0,
        true,
    ));

    let mut world = World::new();

    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext)),
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
        .join(Path::new("scenes/earthmap.jpg"));

    let earth_texture = Arc::new(ImageTexture::new(&path));
    let earth_surface = Arc::new(LambertianMaterial::new_with_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, earth_surface));

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

pub fn rectangle_light_scene(settings: &ImageSettings) -> Scene {
    let mut world = World::new();

    let pertext = Arc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::VectorSmoothing,
        4.0,
        true,
    ));
    let lambertian = Arc::new(LambertianMaterial::new_with_texture(pertext));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        lambertian.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        lambertian,
    )));

    // Note that the light is brighter than (1,1,1). This allows it to be bright enough to light things.
    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    let lookfrom = Point::new(26.0, 3.0, 6.0);
    let lookat = Point::new(0.0, 2.0, 0.0);
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

    Scene::new(world, camera, Color::new(0.0, 0.0, 0.0))
}

/// The "Cornell Box" was introduced in 1984 to model the interaction of light between diffuse surfaces.
/// This function will make the 5 walls and the light of the box.
fn cornell_box() -> World {
    let mut world = World::new();

    let red = Arc::new(LambertianMaterial::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(LambertianMaterial::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(LambertianMaterial::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    world.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    world.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    world.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    world.add(Arc::new(XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white)));

    world
}

pub fn empty_cornell_box(settings: &ImageSettings) -> Scene {
    let world = cornell_box();

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
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

    Scene::new(world, camera, Color::new(0.0, 0.0, 0.0))
}

pub fn cornell_box_with_two_boxes(settings: &ImageSettings) -> Scene {
    let mut world = cornell_box();

    let white = Arc::new(LambertianMaterial::new(Color::new(0.73, 0.73, 0.73)));

    let mut box1: Arc<dyn Hittable> = Arc::new(objects::Box::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    let arcbox1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(arcbox1);

    let mut box2: Arc<dyn Hittable> = Arc::new(objects::Box::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    let arcbox2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(arcbox2);

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
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

    Scene::new(world, camera, Color::new(0.0, 0.0, 0.0))
}

/// # Rendering a Cornell Box with Smoke and Fog Boxes
/// If we replace the two blocks with smoke and fog (dark and light particles), and make the light
/// bigger (and dimmer so it doesn't blow out the scene) for faster convergence:
pub fn cornell_smoke(settings: &ImageSettings) -> Scene {
    let mut world = cornell_box();

    let white = Arc::new(LambertianMaterial::new(Color::new(0.73, 0.73, 0.73)));

    let mut box1: Arc<dyn Hittable> = Arc::new(objects::Box::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(objects::Box::new(
        Point::new(0.0, 0.0, 0.0),
        Point::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::new_with_color(
        box1,
        Color::new(0.0, 0.0, 0.0),
        0.01,
    )));
    world.add(Arc::new(ConstantMedium::new_with_color(
        box2,
        Color::new(1.0, 1.0, 1.0),
        0.01,
    )));

    let lookfrom = Point::new(278.0, 278.0, -800.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
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

    Scene::new(world, camera, Color::new(0.0, 0.0, 0.0))
}

/// # Raytracing: The next week
/// Let's put it all together, with a big thin mist covering everything, and a blue subsurface reflection
/// sphere (we didn't implement that explicitly, but a volume inside a dielectric is what a subsurface
/// material is). The biggest limitation left in the renderer is no shadow rays, but that is why we
/// get caustics and subsurface for free. It's a double-edged design decision.
pub fn rtnextweek_final_scene(settings: &ImageSettings) -> Scene {
    let mut boxes1 = World::new();
    let ground = Arc::new(LambertianMaterial::new(Color::new(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: i32 = 20;
    let w = 100.0;

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(objects::Box::new(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = World::new();

    world.add(Arc::new(boxes1));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point::new(400.0, 400.0, 400.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(LambertianMaterial::new(Color::new(0.7, 0.3, 0.1)));

    world.add(Arc::new(
        MovingSphere::new(center1, center2, 50.0, moving_sphere_material, 0.0, 1.0).unwrap(),
    ));
    world.add(Arc::new(Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    );
    world.add(Arc::new(boundary.clone()));
    world.add(Arc::new(ConstantMedium::new_with_color(
        Arc::new(boundary),
        Color::new(0.2, 0.4, 0.9),
        0.2,
    )));

    let boundary1 = Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::new_with_color(
        boundary1,
        Color::new(1.0, 1.0, 1.0),
        0.0001,
    )));

    let path = std::env::current_dir()
        .unwrap()
        .join(Path::new("./scenes/earthmap.jpg"));
    let emat = Arc::new(LambertianMaterial::new_with_texture(Arc::new(
        ImageTexture::new(&path),
    )));
    world.add(Arc::new(Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    let pertext = Arc::new(PerlinNoiseTexture::new(
        PerlinNoiseOptions::VectorSmoothing,
        0.1,
        true,
    ));
    world.add(Arc::new(Sphere::new(
        Point::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(LambertianMaterial::new_with_texture(pertext)),
    )));

    let mut boxes2 = World::new();
    let white = Arc::new(LambertianMaterial::new(Color::new(0.73, 0.73, 0.73)));
    for i in 0..1000 {
        boxes2.add(Arc::new(Sphere::new(
            Point::random_vector(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(boxes2), 15.0)),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    let lookfrom = Point::new(478.0, 278.0, -600.0);
    let lookat = Point::new(278.0, 278.0, 0.0);
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

    Scene::new(world, camera, Color::new(0.0, 0.0, 0.0))
}
