extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("earth_scene.jpg".to_string(), Scenes::EarthScene);
    });
    eprintln!("{time} seconds to render the image");
}
