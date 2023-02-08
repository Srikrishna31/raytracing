mod camera;
mod configuration;
mod ray;
mod renderer;
mod rtweekend;
mod scenes;
mod vec3;
// Export all the functions structs and constants for use in other crates.
pub mod materials;
pub mod objects;

pub use camera::Camera;
pub use ray::Ray;
// Reexport rtweekend symbols encapsulated in utils, for better naming.
pub mod utils {
    pub use crate::rtweekend::*;
}
pub use configuration::{load_configuration, ImageSettings};
pub use renderer::render;
pub use vec3::{Color, Point, Vec3};
