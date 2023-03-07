extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene(
            "rectangle_light_scene.jpg".to_string(),
            Scenes::RectangleLightScene,
        );
    });
    eprintln!("{time} seconds to render the image");
}
