extern crate scenes;

use timeit::timeit_loops;
use scenes::Scenes;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene("rtnextweek.jpg".to_string(), Scenes::RTNextWeekFinalScene);
    });
    eprintln!("{time} seconds to render the image");
}
