extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("marble_spheres.jpg".to_string(), Scenes::MarbleSpheres);
    });
    eprintln!("{time} seconds to render the image");
}
