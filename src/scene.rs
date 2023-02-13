use crate::objects::World;
use crate::{Camera, Color};

#[derive(Clone)]
pub struct Scene {
    pub(crate) world: World,
    pub(crate) camera: Camera,
    pub(crate) background_color: Color,
}

impl Scene {
    pub fn new(world: World, camera: Camera, background_color: Color) -> Scene {
        Scene {
            world,
            camera,
            background_color,
        }
    }
}
