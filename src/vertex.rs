use glam::{Vec2, Vec3, Vec4};
use std::ops::{Add, Mul, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    pub position: Vec4,
    pub normal: Vec3,
    pub color: Vec3,
    pub uv: Vec2,
}
impl Vertex {
    pub fn new(position: Vec4, normal: Vec3, color: Vec3, uv: Vec2) -> Self {
        Self {
            position,
            normal,
            color,
            uv,
        }
    }
    pub fn new_simple(px: f32, py: f32, pz: f32, r: f32, g: f32, b: f32, u: f32, v: f32) -> Self {
        Self {
            position: Vec4::new(px, py, pz, 1.0),
            normal: Vec3::new(0.0, 0.0, 1.0),
            color: Vec3::new(r, g, b),
            uv: Vec2::new(u, v),
        }
    }
}

impl Add for Vertex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let position = self.position + rhs.position;
        let normal = self.normal + rhs.normal;
        let color = self.color + rhs.color;
        let uv = self.uv + rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Sub for Vertex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        let position = self.position - rhs.position;
        let normal = self.normal - rhs.normal;
        let color = self.color - rhs.color;
        let uv = self.uv - rhs.uv;
        Self::new(position, normal, color, uv)
    }
}

impl Mul<f32> for Vertex {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self {
        let position = self.position * rhs;
        let normal = self.normal * rhs;
        let color = self.color * rhs;
        let uv = self.uv * rhs;
        Self::new(position, normal, color, uv)
    }
}
