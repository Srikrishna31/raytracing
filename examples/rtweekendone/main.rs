extern crate raytracing;

mod scenes;

use raytracing::{load_configuration, render};
use std::rc::Rc;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        let settings = load_configuration().expect("Couldnot read settings");
        // let scene = scenes::rtweekend_one_final_scene();
        // let scene = scenes::scene_with_alternate_viewpoint();
        // let scene =
        //     scenes::rtweekend_one_final_scene_with_moving_spheres_checkered_texture(&settings);
        // let scene = scenes::perlin_textured_spheres(&settings);
        // let scene = scenes::marble_spheres(&settings);
        // let scene = scenes::earth_scene(&settings);
        // let scene = scenes::scene_with_alternate_viewpoint();
        let scene = scenes::rectangle_light_scene(&settings);
        render(settings, scene, |i: f64| eprintln!("{i:.2}% completed"))
    });

    eprintln!("{time} seconds to render the image");
}
