extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene(
            "cornellbox_with_smoke.jpg".to_string(),
            Scenes::CornellBoxWithSmoke,
        );
    });
    eprintln!("{time} seconds to render the image");
}
