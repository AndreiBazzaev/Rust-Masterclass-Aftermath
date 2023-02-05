use glam::{Vec2, Vec3, Mat4};
pub struct RenderWindowSize {
    pub render_width: usize,
    pub render_height: usize,
}

impl RenderWindowSize {
    pub fn with_size(width: usize, height: usize) -> Self {
        Self {
            render_width: width,
            render_height: height,
        }
    }
}
//clockwise
pub fn edge_function(v0: Vec2, v1: Vec2, p: Vec2) -> f32 {
    let v0_p = p - v0;
    let v0_v1 = v1 - v0;
    v0_p.x * v0_v1.y - v0_p.y * v0_v1.x
}
pub fn barycentric_coordinates(
    point: Vec2,
    v0: Vec2,
    v1: Vec2,
    v2: Vec2,
    area: f32,
) -> Option<Vec3> {
    let a = 1.0 / area;
    let m0 = edge_function(v1, v2, point);
    let m1 = edge_function(v2, v0, point);
    let m2 = edge_function(v0, v1, point);

    if (m0 >= 0.0 && m1 >= 0.0 && m2 >= 0.0) || (m0 < 0.0 && m1 < 0.0 && m2 < 0.0) {
        // || (m0 < 0.0 && m1 < 0.0 && m2 < 0.0
        Some(glam::vec3(m0 * a, m1 * a, m2 * a))
    } else {
        None
    }
}
pub fn to_argb8(a: u8, r: u8, g: u8, b: u8) -> u32 {
    let mut argb: u32 = a as u32; //a
    argb = (argb << 8) + r as u32; //r
    argb = (argb << 8) + g as u32; //g
    argb = (argb << 8) + b as u32; //b
    argb
}
pub fn multiply_on_color(original_color: u32, color: Vec3) -> u32 {
    let original_argb: [u8; 4] = unsafe { std::mem::transmute(original_color) };
    let mut argb: u32 = original_argb[0] as u32; //a
    argb = (argb << 8) + (original_argb[1] as f32 * color.x) as u32; //r
    argb = (argb << 8) + (original_argb[2] as f32 * color.y) as u32; //g
    argb = (argb << 8) + (original_argb[3] as f32 * color.z) as u32; //b
    argb
}
pub fn lerp<T>(start: T, end: T, alpha: f32) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Mul<f32, Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    start + (end - start) * alpha
}

pub fn rand_col() -> glam::Vec3 {
    glam::vec3(
        rand::random::<f32>() * 255.0,
        rand::random::<f32>() * 255.0,
        rand::random::<f32>() * 255.0,
    )
}
pub fn map_to_range<T>(v: T, a1: T, a2: T, b1: T, b2: T) -> T
where
    T: std::ops::Sub<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Mul<Output = T>
        + std::ops::Add<Output = T>
        + Copy,
{
    b1 + (v - a1) * (b2 - b1) / (a2 - a1)
}
pub fn minor(
    src: &[f32; 16],
    r0: usize,
    r1: usize,
    r2: usize,
    c0: usize,
    c1: usize,
    c2: usize,
) -> f32 {
    src[4 * r0 + c0] * (src[4 * r1 + c1] * src[4 * r2 + c2] - src[4 * r2 + c1] * src[4 * r1 + c2])
        - src[4 * r0 + c1]
            * (src[4 * r1 + c0] * src[4 * r2 + c2] - src[4 * r2 + c0] * src[4 * r1 + c2])
        + src[4 * r0 + c2]
            * (src[4 * r1 + c0] * src[4 * r2 + c1] - src[4 * r2 + c0] * src[4 * r1 + c1])
}

pub fn cofactor(matrix: &Mat4) -> Mat4 {
    let src: [f32; 16] = matrix.to_cols_array();
    let mut dst: [f32; 16] = [0.0; 16];
    dst[0] = minor(&src, 1, 2, 3, 1, 2, 3);
    dst[1] = -minor(&src, 1, 2, 3, 0, 2, 3);
    dst[2] = minor(&src, 1, 2, 3, 0, 1, 3);
    dst[3] = -minor(&src, 1, 2, 3, 0, 1, 2);
    dst[4] = -minor(&src, 0, 2, 3, 1, 2, 3);
    dst[5] = minor(&src, 0, 2, 3, 0, 2, 3);
    dst[6] = -minor(&src, 0, 2, 3, 0, 1, 3);
    dst[7] = minor(&src, 0, 2, 3, 0, 1, 2);
    dst[8] = minor(&src, 0, 1, 3, 1, 2, 3);
    dst[9] = -minor(&src, 0, 1, 3, 0, 2, 3);
    dst[10] = minor(&src, 0, 1, 3, 0, 1, 3);
    dst[11] = -minor(&src, 0, 1, 3, 0, 1, 2);
    dst[12] = -minor(&src, 0, 1, 2, 1, 2, 3);
    dst[13] = minor(&src, 0, 1, 2, 0, 2, 3);
    dst[14] = -minor(&src, 0, 1, 2, 0, 1, 3);
    dst[15] = minor(&src, 0, 1, 2, 0, 1, 2);
    Mat4::from_cols_array(&dst)
}