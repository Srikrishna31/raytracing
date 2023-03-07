extern crate scenes;

use scenes::Scenes;
use timeit::timeit_loops;

fn main() {
    let time = timeit_loops!(1, {
        scenes::render_scene(
            "rtweekendone_moving_spheres_checkered_texture.jpg".to_string(),
            Scenes::RTWeekendOneFinalSceneWithMovingSpheresCheckeredTexture,
        );
    });
    eprintln!("{time} seconds to render the image");
}
