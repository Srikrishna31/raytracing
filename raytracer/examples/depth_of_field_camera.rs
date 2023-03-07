extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("depth_of_field_camara.jpg".to_string(), Scenes::SceneWithDepthofFieldCamera);
    });
    eprintln!("{time} seconds to render the image");
}
