
use crate::render_object::*;
use crate::utils::*;
use crate::texture::*;
use std::path::Path;
//TODO: make an option
fn load_node( node: &gltf::Node,
    render_object: &mut RenderObject,
    buffers: &[gltf::buffer::Data])
    {
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
            render_object.load_from_gltf(&mesh, &buffers);
     }
     for child_node in node.children(){
        load_node( &child_node,
            render_object,
            buffers,)
     }
}
pub fn load_gltf(path: &Path) -> RenderObject {
    // handle loading textures, cameras, meshes here
    let (document, buffers, images) = gltf::import(path).unwrap();
    let mut render_object = RenderObject::new();
    for image in images {
        let mut data: Vec<u32> = Vec::new();
        let mut depth = 0 as usize;
        
           if image.format ==  gltf::image::Format::R8G8B8 {
                data = image
                    .pixels
                    .chunks(3)
                    .map(|rgb| to_argb8(255, rgb[0], rgb[1], rgb[2]))
                    .collect();
                    depth = 3;
                
            }
            if image.format ==  gltf::image::Format::R8G8B8A8{
                data = image
                    .pixels
                    .chunks(4)
                    .map(|rgba| to_argb8(rgba[3], rgba[2], rgba[1], rgba[0]))
                    .collect();
                    depth = 4;
            }
        

        render_object.textures().push(Some(Texture {
            width: image.width as usize,
            height: image.height as usize,
            data,
            depth,
        }));
    }
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
            // if let Some(mesh) = node.mesh() {
            //     render_object.load_from_gltf(&mesh, &buffers);
            // }
            load_node(&node, &mut render_object, &buffers);
        }
        
    }
    render_object
}
