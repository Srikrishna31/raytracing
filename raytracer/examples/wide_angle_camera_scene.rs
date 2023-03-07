extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene(
            "wide_angle_camera_scene.jpg".to_string(),
            Scenes::WideAngleCameraScene,
        );
    });
    eprintln!("{time} seconds to render the image");
}
