// Is used to reduce size of main
use crate::{vertex::Vertex, RenderObject, mesh_loader::*, Camera, utils::*, renderer::*};
use std::path::Path;

pub struct Scene {
    indices: Vec<u32>,
    vertices: Vec<Vertex>,
    render_objects: Vec<RenderObject>
}
impl Scene {
    pub fn new() -> Self {
        
        let mut new_render_objects: Vec<RenderObject> = vec![];
        for i in 0..=2 as usize {
            for j in 0..=2 as usize {
            let mut render_object = load_gltf(
                Path::new("assets/helmet/Helmet.gltf"),
            );
            render_object.transform().set_translation(glam::vec3(i as f32 * 2.0 - 2.5, j as f32 * 2.0 - 2.5, 0.0));
            new_render_objects.push(render_object);
            }
        }
        let mut render_object = load_gltf(
            Path::new("assets/carp/scene.gltf"),
        );
        render_object.transform().set_translation(glam::vec3(0.0, 8.0, 0.0));
        render_object.transform().set_scale(glam::vec3(0.05, 0.05, 0.05));
        new_render_objects.push(render_object);

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
            .set_rotation(cur_rot * glam::Quat::from_euler(glam::EulerRot::XYZ, 0.05, 0.05, 0.03));
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
        let light_dir = glam::vec3(0.0, 0.0,0.5).normalize();
        for i in 0..= self.render_objects.len() as usize - 1{
            raster_mesh(
                &self.render_objects[i],
                buffer,
                z_buffer,
                render_window_size,
                &light_dir,
            );
        }
    }
}
