extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("marble_spheres.jpg".to_string(), Scenes::MarbleSpheres);
    });
    eprintln!("{time} seconds to render the image");
}
