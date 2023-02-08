use crate::Camera;
use crate::objects::World;

#[derive(Clone)]
pub struct Scene {
    world: World,
    camera: Camera
}

impl Scene {
    pub fn new(world: World, camera: Camera) -> Scene {
        Scene{world, camera}
    }
}
