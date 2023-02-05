use minifb::{Key, Window, WindowOptions};
use std::path::Path;

const WIDTH: usize = 500;
const HEIGHT: usize = 500;

use textured_cube::*;

fn main() {
    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut z_buffer = vec![f32::INFINITY; WIDTH * HEIGHT];
    // Limit to max ~60 fps update rate
    window.limit_update_rate(Some(std::time::Duration::from_micros(16600)));

    // let scene = Scene::new();
    // let mut render_object = RenderObject::from_indices_vertices_texture(
    //     &scene.indices(),
    //     &scene.vertices(),
    //     Path::new("assets/bojan.jpg"),
    // );

    let mut scene = Scene::new_many_helmets();

    let aspect_ratio = WIDTH as f32 / HEIGHT as f32;
    let mut cam_transform = Transform::IDENTITY;
    cam_transform.set_translation(glam::vec3(0.0, 0.0, 2.0));

    let mut camera = Camera {
        aspect_ratio,
        transform: cam_transform,
        f_far: 100.0,
        ..Default::default()
    };
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        clear_screen(&mut buffer, &mut z_buffer, glam::vec3(255.0, 255.0, 0.0));

        camera.move_cam(&window);
        scene.update(&camera);
        scene.draw( &mut buffer,
            &mut z_buffer,
            &RenderWindowSize::with_size(WIDTH, HEIGHT));
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
