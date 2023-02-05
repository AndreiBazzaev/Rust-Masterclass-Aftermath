// is used to reduce main size
use crate::{vertex::Vertex, RenderObject, mesh_loader::*, Camera, utils::*, renderer::*};
use std::path::Path;

pub struct Scene {
    indices: Vec<u32>,
    vertices: Vec<Vertex>,
    render_objects: Vec<RenderObject>
}
impl Scene {
    pub fn new() -> Self {
        Self {
            
            indices: vec![
                // Front face
                2, 1, 0, 0, 3, 2, // Back face
                4, 5, 6, 6, 7, 4, // Left face
                8, 9, 10, 10, 11, 8, // Right face
                14, 13, 12, 12, 15, 14, // Top face
                16, 17, 18, 18, 19, 16, // Bottom face
                22, 21, 20, 20, 23, 22,
            ],

            vertices: vec![
                // Front face
                Vertex::new_simple(-0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 1.0),
                Vertex::new_simple(0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 1.0, 1.0),
                Vertex::new_simple(0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0),
                Vertex::new_simple(-0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0),
                // Back face
                Vertex::new_simple(-0.5, 0.5, -0.5, 0.0, 1.0, 0.0, 0.0, 1.0),
                Vertex::new_simple(0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0),
                Vertex::new_simple(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 0.0),
                Vertex::new_simple(-0.5, -0.5, -0.5, 0.0, 0.0, 0.0, 0.0, 0.0),
                // Left face
                Vertex::new_simple(-0.5, 0.5, 0.5, 0.0, 1.0, 1.0, 0.0, 1.0),
                Vertex::new_simple(-0.5, 0.5, -0.5, 1.0, 0.0, 1.0, 1.0, 1.0),
                Vertex::new_simple(-0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 0.0),
                Vertex::new_simple(-0.5, -0.5, 0.5, 0.0, 0.0, 1.0, 0.0, 0.0),
                // Right face
                Vertex::new_simple(0.5, 0.5, 0.5, 1.0, 1.0, 1.0, 0.0, 1.0),
                Vertex::new_simple(0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 1.0, 1.0),
                Vertex::new_simple(0.5, -0.5, -0.5, 1.0, 1.0, 1.0, 1.0, 0.0),
                Vertex::new_simple(0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 0.0, 0.0),
                // Top face
                Vertex::new_simple(-0.5, 0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new_simple(0.5, 0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0),
                Vertex::new_simple(0.5, 0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0),
                Vertex::new_simple(-0.5, 0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0),
                // Bottom face
                Vertex::new_simple(-0.5, -0.5, 0.5, 1.0, 0.0, 0.0, 0.0, 0.0),
                Vertex::new_simple(0.5, -0.5, 0.5, 1.0, 1.0, 0.0, 1.0, 0.0),
                Vertex::new_simple(0.5, -0.5, -0.5, 0.0, 1.0, 1.0, 1.0, 1.0),
                Vertex::new_simple(-0.5, -0.5, -0.5, 1.0, 1.0, 0.0, 0.0, 1.0),
            ],
            render_objects: vec![],
        }
    }

    pub fn new_many_helmets() -> Self {
        
        let mut new_render_objects: Vec<RenderObject> = vec![];
        for i in 0..=2 as usize {
            for j in 0..=2 as usize {
            let mut render_object = load_gltf_with_texture(
                Path::new("assets/helmet/Helmet.gltf"),
                Path::new("assets/helmet/Default_albedo.png"),
            );
            render_object.transform().set_translation(glam::vec3(i as f32 * 2.0 - 2.5, j as f32 * 2.0 - 2.5, 0.0));
            new_render_objects.push(render_object);
            }
        }
        
        Self {
            indices: vec![],
            vertices: vec![],
            render_objects: new_render_objects,
        }
    }
    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }
    pub fn my_scene_update(&mut self) {
        for i in 0..= self.render_objects.len() as usize - 1 {
            let cur_rot = *self.render_objects[i]
            .transform()
            .rotation();
            self.render_objects[i].transform()
            .set_rotation(cur_rot * glam::Quat::from_euler(glam::EulerRot::XYZ, 0.05, 0.05, 0.0));
        }
    }
    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn update(&mut self, camera: &Camera){
        self.my_scene_update();
        for i in 0..= self.render_objects.len() as usize - 1 {
            self.render_objects[i].update(camera);
        }
    }
    pub fn draw(&mut self,  
        buffer: &mut Vec<u32>,
        z_buffer: &mut Vec<f32>,
        render_window_size: &RenderWindowSize,
    ){
        for i in 0..= self.render_objects.len() as usize - 1{
            raster_mesh(
                &self.render_objects[i],
                buffer,
                z_buffer,
                render_window_size,
            );
        }
    }
}
