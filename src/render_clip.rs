use crate::geometry::*;
use crate::utils::*;
use glam::{Vec4, Vec4Swizzles};
pub enum ClipResult {
    None,
    One(Triangle),
    Two((Triangle, Triangle)),
}

//View Frustum Culling
pub fn cull_triangle_view_frustum(triangle: &Triangle) -> bool {
    // cull tests against the 6 planes
    if triangle.0.vertex.position.x > triangle.0.vertex.position.w
        && triangle.1.vertex.position.x > triangle.1.vertex.position.w
        && triangle.2.vertex.position.x > triangle.2.vertex.position.w
    {
        return true;
    }
    if triangle.0.vertex.position.x < -triangle.0.vertex.position.w
        && triangle.1.vertex.position.x < -triangle.1.vertex.position.w
        && triangle.2.vertex.position.x < -triangle.2.vertex.position.w
    {
        return true;
    }
    if triangle.0.vertex.position.y > triangle.0.vertex.position.w
        && triangle.1.vertex.position.y > triangle.1.vertex.position.w
        && triangle.2.vertex.position.y > triangle.2.vertex.position.w
    {
        return true;
    }
    if triangle.0.vertex.position.y < -triangle.0.vertex.position.w
        && triangle.1.vertex.position.y < -triangle.1.vertex.position.w
        && triangle.2.vertex.position.y < -triangle.2.vertex.position.w
    {
        return true;
    }
    if triangle.0.vertex.position.z > triangle.0.vertex.position.w
        && triangle.1.vertex.position.z > triangle.1.vertex.position.w
        && triangle.2.vertex.position.z > triangle.2.vertex.position.w
    {
        return true;
    }
    if triangle.0.vertex.position.z < 0.0
        && triangle.1.vertex.position.z < 0.0
        && triangle.2.vertex.position.z < 0.0
    {
        return true;
    }

    false
}

pub fn clip_triangle_two(triangle: &Triangle) -> (Triangle, Triangle) {
    // calculate alpha values for getting adjusted vertices
    let alpha_a = (-triangle.0.vertex.position.z)
        / (triangle.1.vertex.position.z - triangle.0.vertex.position.z);
    let alpha_b = (-triangle.0.vertex.position.z)
        / (triangle.2.vertex.position.z - triangle.0.vertex.position.z);

    // interpolate to get v0a and v0b
    let v0_a = lerp(triangle.0.vertex, triangle.1.vertex, alpha_a);
    let v0_b = lerp(triangle.0.vertex, triangle.2.vertex, alpha_b);

    // draw triangles
    let mut result_a = *triangle;
    let mut result_b = *triangle;

    result_a.0.vertex = v0_a;

    result_b.0.vertex = v0_a;
    result_b.1.vertex = v0_b;

    (result_a, result_b)
}

pub fn clip_triangle_one(triangle: &Triangle) -> Triangle {
    // calculate alpha values for getting adjusted vertices
    let alpha_a = (-triangle.0.vertex.position.z)
        / (triangle.2.vertex.position.z - triangle.0.vertex.position.z);
    let alpha_b = (-triangle.1.vertex.position.z)
        / (triangle.2.vertex.position.z - triangle.1.vertex.position.z);

    let mut result = *triangle;
    // interpolate to get v0a and v0b
    result.0.vertex = lerp(triangle.0.vertex, triangle.2.vertex, alpha_a);
    result.1.vertex = lerp(triangle.1.vertex, triangle.2.vertex, alpha_b);
    result.2.vertex = triangle.2.vertex;
    result
}
pub fn cull_triangle_backface(pos_0: &Vec4, pos_1: &Vec4, pos_2: &Vec4) -> bool {
    let normal = (pos_1.xyz() - pos_0.xyz())
        .cross(pos_2.xyz() - pos_0.xyz())
        .normalize();

    let view_dir0 = pos_0.xyz().normalize();
    let view_dir1 = pos_1.xyz().normalize();
    let view_dir2 = pos_2.xyz().normalize();

    let dot0 = view_dir0.dot(normal);
    let dot1 = view_dir1.dot(normal);
    let dot2 = view_dir2.dot(normal);
    dot0 < 0.0 && dot1 < 0.0 && dot2 < 0.0
}

pub fn clip_cull_triangle(triangle: &Triangle) -> ClipResult {
    if cull_triangle_view_frustum(triangle) {
        ClipResult::None
    } else {
        // clipping routines
        if triangle.0.vertex.position.z < 0.0 {
            if triangle.1.vertex.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(triangle))
            } else if triangle.2.vertex.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::ACB)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::ACB)))
            }
        } else if triangle.1.vertex.position.z < 0.0 {
            if triangle.2.vertex.position.z < 0.0 {
                ClipResult::One(clip_triangle_one(&triangle.reorder(VerticesOrder::BCA)))
            } else {
                ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::BAC)))
            }
        } else if triangle.2.vertex.position.z < 0.0 {
            ClipResult::Two(clip_triangle_two(&triangle.reorder(VerticesOrder::CBA)))
        } else {
            // no near clipping necessary
            //return original
            ClipResult::One(*triangle)
        }
    }
}
