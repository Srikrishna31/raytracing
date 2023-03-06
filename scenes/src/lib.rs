extern crate raytracer;
mod scenes;

use raytracer::{load_configuration, render, ImageSettings};
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

pub use scenes::*;

pub enum Scenes {
    DielectricShinySphere,
    SceneWithHollowGlassSphere,
    WideAngleCameraScene,
    SceneWithAlternateViewPoint,
    SceneWithDepthofFieldCamera,
    RTWeekendOneFinalScene,
    RTWeekendOneFinalSceneWithMovingSpheres,
    RTWeekendOneFinalSceneWithMovingSpheresCheckeredTexture,
    TwoCheckeredSpheres,
    PerlinTexturedSpheres,
    PerlinSmoothedTexturedSpheres,
    MarbleSpheres,
    EarthScene,
    RectangleLightScene,
    EmptyCornellBox,
    CornellBoxWithTwoBoxes,
    CornellBoxWithSmoke,
    RTNextWeekFinalScene
}

pub fn render_scene(filename: String, function: Scenes) {
    let mut settings = load_configuration().expect("Couldnot read settings");

    let total = 100;
    let pb = ProgressBar::new(total);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {msg}",
        )
            .unwrap()
            .progress_chars("#>-"),
    );

    settings.path = std::env::current_dir()
        .unwrap()
        .join(Path::new(filename.as_str()))
        .into_os_string()
        .into_string()
        .expect("Couldnot build path to file");
    // let scene = scenes::rtweekend_one_final_scene(&settings);
    let scene = match function {
        Scenes::EarthScene => earth_scene(&settings),
        Scenes::CornellBoxWithSmoke => cornell_smoke(&settings),
        Scenes::CornellBoxWithTwoBoxes => cornell_box_with_two_boxes(&settings),
        Scenes::PerlinSmoothedTexturedSpheres => perlin_smoothed_textured_spheres(&settings),
        //Scenes::DielectricShinySphere => scene_with_dielectric_and_shiny_sphere(),
        Scenes::EmptyCornellBox => empty_cornell_box(&settings),
        Scenes::RectangleLightScene => rectangle_light_scene(&settings),
        Scenes::RTWeekendOneFinalScene => rtweekend_one_final_scene(&settings),
        Scenes::RTWeekendOneFinalSceneWithMovingSpheres => rtweekend_one_final_scene_with_moving_spheres(&settings),
        Scenes::RTWeekendOneFinalSceneWithMovingSpheres => rtweekend_one_final_scene_with_moving_spheres(&settings),
        Scenes::RTNextWeekFinalScene => rtnextweek_final_scene(&settings),
        _ => earth_scene(&settings)
    };

    let scene = scenes::scene_with_alternate_viewpoint();
    // let scene =
    //     scenes::rtweekend_one_final_scene_with_moving_spheres_checkered_texture(&settings);
    // let scene = scenes::perlin_textured_spheres(&settings);
    // let scene = scenes::marble_spheres(&settings);
    // let scene = scenes::earth_scene(&settings);
    // let scene = scenes::scene_with_alternate_viewpoint();
    // let scene = scenes::rectangle_light_scene(&settings);
    // let scene = scenes::empty_cornell_box(&settings);
    // let scene = scenes::cornell_box_with_two_boxes(&settings);
    // let scene = scenes::cornell_smoke(&settings);
    // let scene = scenes::rtnextweek_final_scene(&settings);

    render(settings, scene, |i: f64| {
        pb.set_position(i as u64);
        pb.set_message(format!("{i:.2}%"));
    });

    pb.finish_with_message("Done!");

}
