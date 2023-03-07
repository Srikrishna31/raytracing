extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("cornellbox.jpg".to_string(), Scenes::EmptyCornellBox);
    });
    eprintln!("{time} seconds to render the image");
}
