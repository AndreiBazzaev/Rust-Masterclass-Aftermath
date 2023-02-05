// Includes all other files for the project
pub mod buffer2d;
pub mod camera;
pub mod geometry;
pub mod mesh_loader;
pub mod my_scene;
pub mod render_clip;
pub mod render_object;
pub mod renderer;
pub mod texture;
pub mod transform;
pub mod utils;
pub mod vertex;
pub use {
    camera::Camera, geometry::*, mesh_loader::*, my_scene::*, render_clip::*, render_object::*,
    renderer::*, texture::Texture, transform::Transform, utils::*, vertex::Vertex,
};
