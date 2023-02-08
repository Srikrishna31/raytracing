use crate::objects::World;
use crate::Camera;

#[derive(Clone)]
pub struct Scene {
    pub(crate) world: World,
    pub(crate) camera: Camera,
}

impl Scene {
    pub fn new(world: World, camera: Camera) -> Scene {
        Scene { world, camera }
    }
}
