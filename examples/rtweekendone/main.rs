extern crate raytracing;

mod scenes;

use raytracing::{load_configuration, render};

use std::rc::Rc;

fn main() {
    let settings = load_configuration().expect("Couldnot read settings");
    // let scene = scenes::rtweekend_one_final_scene();
    let scene = scenes::scene_with_alternate_viewpoint();
    render(settings, scene, |i: f64| eprintln!("{i:.2}% completed"));
}
