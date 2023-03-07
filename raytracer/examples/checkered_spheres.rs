extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("checkered_spheres.jpg".to_string(), Scenes::TwoCheckeredSpheres);
    });
    eprintln!("{time} seconds to render the image");
}
