use crate::vertex::Vertex;
use glam::{Vec2, Vec4};


// Stores information needed in "pixel shader"
#[derive(Debug, Copy, Clone)]
pub struct PixelSHaderVertex {
    pub vertex: Vertex,
    pub ndc: Vec4, // NDC space coords
    pub rec: f32,  // used for proper texturing
    pub sc: Vec2,  // screen coords
}
impl PixelSHaderVertex {
    pub fn new() -> Self {
        Self {
            vertex: Vertex {
                position: (glam::vec4(0.0, 0.0, 0.0, 1.0)),
                normal: (glam::vec3(0.0, 0.0, 1.0)),
                color: (glam::vec3(0.0, 0.0, 0.0)),
                uv: (glam::vec2(0.0, 0.0)),
            },
            ndc: Vec4::new(0.0, 0.0, 0.0, 0.0),
            rec: 0.0,
            sc: Vec2::new(0.0, 0.0),
        }
    }
}
pub enum VerticesOrder {
    ABC,
    ACB,
    BAC,
    BCA,
    CAB,
    CBA,
}
#[derive(Debug, Copy, Clone)]
pub struct Triangle(
    pub PixelSHaderVertex,
    pub PixelSHaderVertex,
    pub PixelSHaderVertex,
);
impl Triangle {
    pub fn new(v0: PixelSHaderVertex, v1: PixelSHaderVertex, v2: PixelSHaderVertex) -> Self {
        Self(v0, v1, v2)
    }
    // Used to keep triangle vertices in the same orientation after lipping them with near plane
    pub fn reorder(&self, order: VerticesOrder) -> Self {
        match order {
            VerticesOrder::ABC => *self,
            VerticesOrder::ACB => Self::new(self.0, self.2, self.1),
            VerticesOrder::BAC => Self::new(self.1, self.0, self.2),
            VerticesOrder::BCA => Self::new(self.1, self.2, self.0),
            VerticesOrder::CAB => Self::new(self.2, self.0, self.1),
            VerticesOrder::CBA => Self::new(self.2, self.1, self.0),
        }
    }
}

// Used to lower iteration steps in "pixel shader"
pub struct BoundingBox2D {
    pub left: f32,
    pub right: f32,
    pub top: f32,
    pub bottom: f32,
}

pub fn get_triangle_bounding_box_2d(positions: &[Vec2; 3]) -> BoundingBox2D {
    let left = positions[0].x.min(positions[1].x).min(positions[2].x);
    let right = positions[0].x.max(positions[1].x).max(positions[2].x);
    let top = positions[0].y.min(positions[1].y).min(positions[2].y);
    let bottom = positions[0].y.max(positions[1].y).max(positions[2].y);

    BoundingBox2D {
        left,
        right,
        top,
        bottom,
    }
}
