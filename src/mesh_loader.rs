use crate::render_object::*;
use std::path::Path;
//TODO: make an option
pub fn load_gltf_with_texture(path: &Path, texture_path: &Path) -> RenderObject {
    // handle loading textures, cameras, meshes here
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                node.index(),
                node.children().count(),
                node.camera(),
                node.mesh().is_some(),
                node.transform(),
            );
            println!(
                "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                node.index(),
                node.transform().decomposed().0,
                node.transform().decomposed().1,
                node.transform().decomposed().2,
                
            );
            if let Some(mesh) = node.mesh() {
                return RenderObject::load_from_gltf_with_texture(&mesh, &buffers, texture_path);
            }
        }
    }

    RenderObject::new()
}

pub fn load_gltf(path: &Path) -> RenderObject {
    // handle loading textures, cameras, meshes here
    let (document, buffers, _images) = gltf::import(path).unwrap();

    for scene in document.scenes() {
        for node in scene.nodes() {
            println!(
                "Node #{} has {} children, camera: {:?}, mesh: {:?}, transform: {:?}",
                node.index(),
                node.children().count(),
                node.camera(),
                node.mesh().is_some(),
                node.transform(),
            );
            println!(
                "Node #{} has transform: trans {:?}, rot {:?}, scale {:?},",
                node.index(),
                node.transform().decomposed().0,
                node.transform().decomposed().1,
                node.transform().decomposed().2,
            );
            if let Some(mesh) = node.mesh() {
                return RenderObject::load_from_gltf(&mesh, &buffers);
            }
        }
    }

    RenderObject::new()
}
