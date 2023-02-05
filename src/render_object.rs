use crate::camera::Camera;
use crate::texture::Texture;
use crate::transform::Transform;
use crate::vertex::Vertex;
use glam::{Vec2, Vec3};
use std::{path::Path};
#[derive(Debug, Clone)]
pub struct CBuffer {
    pub m: glam::Mat4,
    pub v: glam::Mat4,
    pub mv: glam::Mat4,
    pub mvp: glam::Mat4,
}
impl CBuffer {
    pub fn new() -> Self {
        Self {
            m: glam::Mat4::IDENTITY,
            v: glam::Mat4::IDENTITY,
            mv: glam::Mat4::IDENTITY,
            mvp: glam::Mat4::IDENTITY,
        }
    }
}
pub struct Primitive {
    indices: Vec<u32>,
    vertices: Vec<Vertex>,
    texture: Option<Texture>,
}
pub struct RenderObject {
    transform: Transform,
    cbuffer: CBuffer,
    primitives: Vec<Primitive>,
}

impl Primitive {
    pub fn new() -> Self {
        Self {
            indices: Vec::new(),
            vertices: Vec::new(),
            texture: Option::None,
        }
    }

    pub fn indices(&self) -> &Vec<u32> {
        &self.indices
    }

    pub fn vertices(&self) -> &Vec<Vertex> {
        &self.vertices
    }
    pub fn texture(&self) -> &Option<Texture> {
        &self.texture
    }
    pub fn from_indices_vertices(new_indices: &[u32], new_vertices: &[Vertex]) -> Self {
        let mut primitive = Primitive::new();
        primitive.push_indices_vertices(new_indices, new_vertices);
        primitive
    }
    pub fn from_indices_vertices_texture(
        new_indices: &[u32],
        new_vertices: &[Vertex],
        texture_path: &Path,
    ) -> Self {
        let mut primitive = Primitive::new();
        primitive.push_indices_vertices(new_indices, new_vertices);
        primitive.texture = Some(Texture::load(texture_path));
        primitive
    }
    fn push_indices_vertices(&mut self, new_indices: &[u32], new_vertices: &[Vertex]) {
        let offset = self.vertices.len() as u32;
        let indices: Vec<u32> = new_indices.iter().map(|id| *id + offset).collect();
        self.indices.extend_from_slice(&indices);
        self.vertices.extend_from_slice(new_vertices);
    }

    pub fn add_section_from_buffers(
        &mut self,
        indices: &[u32],
        positions: &[Vec3],
        normals: &[Vec3],
        colors: &[Vec3],
        uvs: &[Vec2],
    ) {
        self.indices.extend_from_slice(indices);

        let has_uvs = !uvs.is_empty();
        let has_colors = !colors.is_empty();

        for i in 0..positions.len() {
            let vertex = Vertex::new(
                positions[i].extend(1.0),
                normals[i],
                if has_colors { colors[i] } else { Vec3::ONE },
                if has_uvs { uvs[i] } else { Vec2::ZERO },
            );
            self.vertices.push(vertex)
        }
    }
}
impl RenderObject {
    pub fn new() -> Self {
        Self {
            primitives: Vec::new(),
            transform: Transform::IDENTITY,
            cbuffer: CBuffer::new(),
        }
    }
    pub fn cbuffer(&self) -> &CBuffer {
        &self.cbuffer
    }

    pub fn transform(&mut self) -> &mut Transform {
        &mut self.transform
    }
    pub fn primitives(&self) -> &Vec<Primitive> {
        &&self.primitives
    }
    pub fn update(&mut self, camera: &Camera) {
        self.cbuffer.m = self.transform.model_mat();
        self.cbuffer.v = camera.view();
        self.cbuffer.mv = self.cbuffer.v * self.cbuffer.m;
        self.cbuffer.mvp = camera.projection() * self.cbuffer.mv;
    }
    pub fn load_from_gltf_with_texture(
        mesh: &gltf::Mesh,
        buffers: &[gltf::buffer::Data],
        texture_path: &Path,
    ) -> RenderObject {
        let mut render_object = RenderObject::load_from_gltf(mesh, buffers);
        for prim in 0..render_object.primitives.len(){
            render_object.primitives[prim].texture = Some(Texture::load(texture_path));
        }
        
        render_object
    }

    pub fn load_from_gltf(mesh: &gltf::Mesh, buffers: &[gltf::buffer::Data]) -> RenderObject {
        let mut positions: Vec<Vec3> = Vec::new();
        let mut tex_coords: Vec<Vec2> = Vec::new();
        let mut normals: Vec<Vec3> = Vec::new();
        let mut indices = vec![];
        // TODO: handle errors
        let mut result = RenderObject::new();
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(indices_reader) = reader.read_indices() {
                indices_reader.into_u32().for_each(|i| indices.push(i));
            }
            if let Some(positions_reader) = reader.read_positions() {
                positions_reader.for_each(|p| positions.push(Vec3::new(p[0], p[1], p[2])));
            }
            if let Some(normals_reader) = reader.read_normals() {
                normals_reader.for_each(|n| normals.push(Vec3::new(n[0], n[1], n[2])));
            }
            if let Some(tex_coord_reader) = reader.read_tex_coords(0) {
                tex_coord_reader
                    .into_f32()
                    .for_each(|tc| tex_coords.push(Vec2::new(tc[0], tc[1])));
            }
           
            let colors: Vec<Vec3> = positions.iter().map(|_| Vec3::ONE).collect();
            println!("Num indices: {:?}", indices.len());
            println!("tex_coords: {:?}", tex_coords.len());
            println!("positions: {:?}", positions.len());
            let mut render_primintive = Primitive::new(); 
            render_primintive.add_section_from_buffers(&indices, &positions, &normals, &colors, &tex_coords);
            result.primitives.push(render_primintive);
        }
        result
    }
}
