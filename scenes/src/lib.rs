extern crate raytracer;
mod scenes;

use indicatif::{ProgressBar, ProgressStyle};
use raytracer::{load_configuration, render, ImageSettings, Scene};
use std::path::Path;

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
    RTNextWeekFinalScene,
}

pub fn get_scenes() -> Vec<&'static str> {
    vec![
        "DielectricShinySphere",
        "SceneWithHollowGlassSphere",
        "WideAngleCameraScene",
        "SceneWithAlternativeViewPoint",
        "RTWeekendOneFinalScene",
        "RTWeekendOneFinalSceneWithMovingSpheres",
        "RTWeekendOneFinalSceneWithMovingSpheresCheckeredTexture",
        "TwoCheckeredSpheres",
        "PerlinTexturedSpheres",
        "PerlinSmoothedTexturedSpheres",
        "MarbleSpheres",
        "EarthScene",
        "RectangleLightScene",
        "EmptyCornellBox",
        "CornellBoxWithTwoBoxes",
        "CornellBoxWithSmoke",
        "RTNextWeekFinalScene"
    ]
}
/// This is the common code required for all the examples, which is why it is abstracted into a function,
/// so that the examples code can be minimal.
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

    let scene =scene(function, &settings);

    render(settings, scene, |i: f64| {
        pb.set_position(i as u64);
        pb.set_message(format!("{i:.2}%"));
    });

    pb.finish_with_message("Done!");
}


fn scene(scn: Scenes, settings: &ImageSettings) -> Scene {
    match scn {
        Scenes::EarthScene => earth_scene(&settings),
        Scenes::CornellBoxWithSmoke => cornell_smoke(&settings),
        Scenes::CornellBoxWithTwoBoxes => cornell_box_with_two_boxes(&settings),
        Scenes::PerlinSmoothedTexturedSpheres => perlin_smoothed_textured_spheres(&settings),
        Scenes::EmptyCornellBox => empty_cornell_box(&settings),
        Scenes::RectangleLightScene => rectangle_light_scene(&settings),
        Scenes::RTWeekendOneFinalScene => rtweekend_one_final_scene(&settings),
        Scenes::RTWeekendOneFinalSceneWithMovingSpheres => {
            rtweekend_one_final_scene_with_moving_spheres(&settings)
        }
        Scenes::RTWeekendOneFinalSceneWithMovingSpheresCheckeredTexture => {
            rtweekend_one_final_scene_with_moving_spheres_checkered_texture(&settings)
        }
        Scenes::RTNextWeekFinalScene => rtnextweek_final_scene(&settings),
        Scenes::MarbleSpheres => marble_spheres(&settings),
        Scenes::SceneWithHollowGlassSphere
        | Scenes::DielectricShinySphere
        | Scenes::SceneWithAlternateViewPoint => earth_scene(&settings),
        Scenes::PerlinTexturedSpheres => perlin_textured_spheres(&settings),
        Scenes::WideAngleCameraScene => scene_for_wide_angle_camera(),
        Scenes::SceneWithDepthofFieldCamera => scene_with_depth_of_field_camera(),
        Scenes::TwoCheckeredSpheres => two_checkered_spheres(&settings),
    }
}