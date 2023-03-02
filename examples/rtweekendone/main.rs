extern crate raytracing;

mod scenes;

use raytracing::{load_configuration, render};
use std::path::Path;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        let mut settings = load_configuration().expect("Couldnot read settings");
        settings.path = std::env::current_dir()
            .unwrap()
            .join(Path::new("image_alternate_viewpoint_rel.jpg"))
            .into_os_string()
            .into_string()
            .expect("Couldnot build path to file");
        // let scene = scenes::rtweekend_one_final_scene();
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

        render(settings, scene, |i: f64| eprintln!("{i:.2}% completed"))
    });

    eprintln!("{time} seconds to render the image");
}
