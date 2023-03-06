extern crate scenes;

use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("rtnextweek_parallel.jpg", "render");
    });
    eprintln!("{time} seconds to render the image");
}
