use crate::buffer2d::*;
use crate::utils::*;
use stb_image;
use std::path::Path;
use glam::Vec4;
pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u32>,
    pub depth: usize,
}

impl Texture {
    pub fn load(path: &Path) -> Self {
        let decoded_image = stb_image::image::load(path);
        if let stb_image::image::LoadResult::ImageU8(image) = decoded_image {
            let data = (0..image.data.len() / 3)
                .map(|id| {
                    to_argb8(
                        255,
                        image.data[id * 3],
                        image.data[id * 3 + 1],
                        image.data[id * 3 + 2],
                    )
                })
                .collect();
            Self {
                width: image.width,
                height: image.height,
                data,
                depth: image.depth,
            }
        } else {
            panic!("Unsupported texture type");
        }
    }
    pub fn uv_to_index(&self, u: f32, v: f32) -> usize {
        let (u, v) = (u * self.width as f32, v * self.height as f32);
        coords_to_index(
            (u as usize) % self.width,
            (v as usize) % self.height,
            self.width,
        )
    }
    pub fn argb_at_uv(&self, u: f32, v: f32) -> u32 {
        let id = self.uv_to_index(u, v);
        if id < self.data.len() {
            self.data[id]
        } else {
            to_argb8(0, 0, 0, 0)
        }
    }
    pub fn color_vec_at_uv(&self, u: f32, v: f32) -> Vec4 {
        let original_argb: [u8; 4] = unsafe { std::mem::transmute(self.argb_at_uv(u, v)) };
        glam::vec4(original_argb[0] as f32, original_argb[1] as f32,original_argb[2] as f32,original_argb[3] as f32)
    }
}
