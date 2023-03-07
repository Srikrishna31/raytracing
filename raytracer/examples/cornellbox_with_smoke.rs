extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("cornellbox_with_smoke.jpg".to_string(), Scenes::CornellBoxWithSmoke);
    });
    eprintln!("{time} seconds to render the image");
}
