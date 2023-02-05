use crate::buffer2d::*;
use crate::geometry::*;
use crate::render_clip::*;
use crate::render_object::*;
use crate::texture::*;
use crate::utils::*;
use crate::Vertex;
use glam::{Vec2, Vec3, Vec4,  Vec4Swizzles};

pub fn clear_screen(buffer: &mut Vec<u32>, z_buffer: &mut Vec<f32>, clear_color: Vec3) {
    for (i, pixel) in buffer.iter_mut().enumerate() {
        //buffer[i] = clear_color;
        *pixel = to_argb8(
            255,
            clear_color.x as u8,
            clear_color.y as u8,
            clear_color.z as u8,
        );
        z_buffer[i] = f32::INFINITY;
    }
}
pub fn clip_pass(
    vert: &PixelSHaderVertex,
    render_window_size: &RenderWindowSize,
) -> PixelSHaderVertex {
    let mut ps_vert = *vert;
    ps_vert.rec = 1.0 / vert.vertex.position.w;
    ps_vert.ndc = glam::vec4(
        vert.vertex.position.x,
        vert.vertex.position.y,
        vert.vertex.position.z,
        vert.vertex.position.w,
    ) * ps_vert.rec;
    ps_vert.vertex = vert.vertex * ps_vert.rec;
    ps_vert.sc = glam::vec2(
        map_to_range(
            ps_vert.ndc.x,
            -1.0,
            1.0,
            0.0,
            render_window_size.render_width as f32,
        ),
        map_to_range(
            -ps_vert.ndc.y,
            -1.0,
            1.0,
            0.0,
            render_window_size.render_height as f32,
        ),
    );
    ps_vert
}
pub fn clip_pass_triangle(tr: &mut Triangle, render_window_size: &RenderWindowSize) {
    tr.0 = clip_pass(&tr.0, render_window_size);
    tr.1 = clip_pass(&tr.1, render_window_size);
    tr.2 = clip_pass(&tr.2, render_window_size);
}

pub fn is_facing_cam(
    vertex_0: &Vertex,
    vertex_1: &Vertex,
    vertex_2: &Vertex,
    cbuffer: &CBuffer,
) -> bool {
    let world_coord_0 = cbuffer.mv * vertex_0.position;
    let world_coord_1 = cbuffer.mv * vertex_1.position;
    let world_coord_2 = cbuffer.mv * vertex_2.position;
    cull_triangle_backface(&world_coord_0, &world_coord_1, &world_coord_2)
}

pub fn vertex_pass(vert: &Vertex, cbuffer: &CBuffer) -> PixelSHaderVertex {
    let mut ps_vert = PixelSHaderVertex::new();
    let vert_pass_coord = cbuffer.mvp * vert.position;
    ps_vert.vertex = *vert;
    ps_vert.vertex.position = vert_pass_coord;
    ps_vert
}
pub fn vertex_pass_triangle(vert1: &Vertex,vert2: &Vertex, vert3: &Vertex, cbuffer: &CBuffer) -> Triangle {
    Triangle(vertex_pass(vert1, cbuffer),
    vertex_pass(vert2, cbuffer),
    vertex_pass(vert3, cbuffer))
}
pub fn draw_if_in_triangle(
    cur_pos: &[i32],
    tr: &Triangle,
    texture: &Option<Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    render_window_size: &RenderWindowSize,
) -> bool {
    let coords = glam::vec2(cur_pos[0] as f32, cur_pos[1] as f32) + 0.5;
    let pixel_id = coords_to_index(
        cur_pos[0] as usize,
        cur_pos[1] as usize,
        render_window_size.render_width,
    );

    let area = edge_function(tr.0.sc, tr.1.sc, tr.2.sc);
    
    if let Some(bary) = barycentric_coordinates(coords, tr.0.sc, tr.1.sc, tr.2.sc, area){

        let perspective_correction =
            1.0 / (bary.x * tr.0.rec + bary.y * tr.1.rec + bary.z * tr.2.rec);

        let depth = bary.x * tr.0.ndc.z + bary.y * tr.1.ndc.z + bary.z * tr.2.ndc.z;

        if depth < z_buffer[pixel_id] {
            
            z_buffer[pixel_id] = depth;
            let mut color = to_argb8(255, 255, 255, 255);

            if let Some(tex) = texture {
                let tex_coords = (bary.x * tr.0.vertex.uv + bary.y * tr.1.vertex.uv + bary.z * tr.2.vertex.uv)* perspective_correction;

                let normal = ((bary.x * tr.0.vertex.normal + bary.y * tr.1.vertex.normal + bary.z * tr.2.vertex.normal) * perspective_correction).normalize();
                let n_dot_l = normal.dot(Vec3::ONE.normalize());
                let ambient = 0.2;
                let shading_value: Vec4 = tex.color_vec_at_uv(tex_coords.x, tex_coords.y) * n_dot_l + ambient;

                //color = to_argb8(255, (n_dot_l * 255.0) as u8 , (n_dot_l * 255.0 )as u8, (n_dot_l * 255.0 )as u8);

                //color = tex.argb_at_uv(tex_coords.x, tex_coords.y);
                
                color = to_argb8((shading_value.w) as u8, (shading_value.z) as u8,(shading_value.y) as u8, (shading_value.x) as u8);
            }
            buffer[pixel_id] = color;
        }
    
        true
    } else {
        
        //buffer[pixel_id] = to_argb8(255, 255, 0, 0);
        false
    }

}
pub fn raster_triangle(
    tr: &Triangle,
    texture: &Option<Texture>,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    render_window_size: &RenderWindowSize,
) {
    if let Some(bb) = triangle_screen_bounding_box(&[tr.0.sc, tr.1.sc, tr.2.sc], render_window_size)
    {
        // AABB traversal

        // for y in (bb.top as usize)..=bb.bottom as usize {
        //     for x in (bb.left as usize)..=bb.right as usize {
        //         let coords: [i32; 2] = [x as i32, y as i32];
        //         draw_if_in_triangle(&coords, tr, texture, buffer, z_buffer, render_window_size);
        //     }
        // }

        //Zig-Zag traversal
        let mut dir_x = -1 as i32;
        let mut cur_pos: [i32; 2] = [bb.left as i32 + 1, bb.top as i32];
        let mut visited_lr = vec![false, false];
        let mut was_in_triangle = false;

        while cur_pos[1] <= bb.bottom as i32 {

            cur_pos[0] += dir_x;
            //AABB check
            if cur_pos[0] < bb.left as i32 || cur_pos[0] > bb.right as i32 {

                //If we've already checked a row
                if visited_lr[1] || visited_lr[0] {
                  
                    //I tried to fix it for 2 days. I give up, idk, what causes  12 pixels on the screen not being 
                    //iterated and redndered, I did everything I could think of and failed. This is the solution, which fixes 
                    //the porvlem in a dumb way even though it is not that bad performance wise 
                    if was_in_triangle == false{
                        for x in (bb.left as usize)..=bb.right as usize {
                            let coords: [i32; 2] = [x as i32, cur_pos[1] as i32];
                            draw_if_in_triangle(&coords,tr,texture,  buffer, z_buffer,  render_window_size);
                        }
                    }


                    cur_pos[1] += 1;
                    visited_lr[0] = !visited_lr[0];
                    visited_lr[1] = !visited_lr[1];
                    dir_x = -dir_x;
                    was_in_triangle = false;
                } 
                //If we haven't checked a row
                else {
                    if dir_x < 0 {
                        visited_lr[0] = true;
                    }
                    if dir_x > 0 {
                        visited_lr[1] = true;
                    }
                    was_in_triangle = false;
                    dir_x = -dir_x;
                }
                if cur_pos[1] > bb.bottom as i32 {
                    break;
                }
                continue;
            }

            if draw_if_in_triangle(
                &cur_pos,
                tr,
                texture,
                buffer,
                z_buffer,
                render_window_size,
            ) {
                was_in_triangle = true;
            } else {
                // Change direction when reach empty pixel from one side of the triangle
                if dir_x < 0 {
                    if visited_lr[1] == false || (visited_lr[1] == true && was_in_triangle) {
                        visited_lr[0] = true;
                        dir_x = -dir_x;
                    }
                } else {
                    if visited_lr[0] == false || (visited_lr[0] == true && was_in_triangle) {
                        visited_lr[1] = true;
                        dir_x = -dir_x;
                    }
                }
                // If we went from one side of triangle to the other - go down
                if visited_lr[0] == true && visited_lr[1] == true {
                    dir_x = -dir_x;
                    cur_pos[1] += 1;
                    visited_lr[0] = false;
                    visited_lr[1] = false;
                    cur_pos[0] -= dir_x;
                    was_in_triangle = false;
                }
            }
        }
    }
}
pub fn raster_mesh(
    render_object: &RenderObject,
    buffer: &mut Vec<u32>,
    z_buffer: &mut Vec<f32>,
    render_window_size: &RenderWindowSize,
) {
    for i in (0..render_object.indices().len()).step_by(3) {
        let first_vert_id = render_object.indices()[i] as usize;
        let second_vert_id = render_object.indices()[i + 1] as usize;
        let third_vert_id = render_object.indices()[i + 2] as usize;

        if !is_facing_cam(
            &render_object.vertices()[first_vert_id],
            &render_object.vertices()[second_vert_id],
            &render_object.vertices()[third_vert_id],
            &render_object.cbuffer(),
        ) {
            continue;
        }

        let mut render_poly = 
        vertex_pass_triangle(
        &render_object.vertices()[first_vert_id],
        &render_object.vertices()[second_vert_id], 
        &render_object.vertices()[third_vert_id],
        render_object.cbuffer());

        // let model_view_matrix_inv_transpose = render_object.cbuffer().mv.transpose();
       
        //  render_poly.0.vertex.normal = (render_object.cbuffer().m * render_poly.0.vertex.normal.extend(0.0)).normalize().xyz();
        //  render_poly.1.vertex.normal = (render_object.cbuffer().m * render_poly.1.vertex.normal.extend(0.0)).normalize().xyz();
        //  render_poly.1.vertex.normal = (render_object.cbuffer().m  * render_poly.2.vertex.normal.extend(0.0)).normalize().xyz();
        let cof_mat = cofactor(&render_object.cbuffer().m );
        render_poly.0.vertex.normal = (cof_mat * render_poly.0.vertex.normal.extend(0.0)).xyz();
        render_poly.1.vertex.normal = (cof_mat * render_poly.1.vertex.normal.extend(0.0)).xyz();
        render_poly.2.vertex.normal = (cof_mat * render_poly.2.vertex.normal.extend(0.0)).xyz();

        match clip_cull_triangle(&render_poly) {
            ClipResult::None => {}
            ClipResult::One(mut tri) => {
                clip_pass_triangle(&mut tri, render_window_size);

                raster_triangle(
                    &tri,
                    render_object.texture(),
                    buffer,
                    z_buffer,
                    render_window_size,
                );
            }
            ClipResult::Two(mut tri) => {
                clip_pass_triangle(&mut tri.0, render_window_size);
                raster_triangle(
                    &tri.0,
                    render_object.texture(),
                    buffer,
                    z_buffer,
                    render_window_size,
                );
                clip_pass_triangle(&mut tri.1, render_window_size);
                raster_triangle(
                    &tri.1,
                    render_object.texture(),
                    buffer,
                    z_buffer,
                    render_window_size,
                );
            }
        }
    }
}

pub fn triangle_screen_bounding_box(
    positions: &[Vec2; 3],
    render_window_size: &RenderWindowSize,
) -> Option<BoundingBox2D> {
    let bb = get_triangle_bounding_box_2d(positions);

    if bb.left >= render_window_size.render_width as f32
        || bb.right < 0.0
        || bb.top >= render_window_size.render_height as f32
        || bb.bottom < 0.0
    {
        None
    } else {
        let left = bb.left.max(0.0);
        let right = bb.right.min(render_window_size.render_width as f32 - 1.0);
        let top = bb.top.max(0.0);
        let bottom = bb.bottom.min(render_window_size.render_height as f32 - 1.0);

        Some(BoundingBox2D {
            left,
            right,
            top,
            bottom,
        })
    }
}
