use crate::transform::Transform;
use glam::{Mat4, Vec2};
use minifb::{Key, MouseMode, Window};

pub struct Camera {
    pub f_near: f32,
    pub f_far: f32,
    pub fov: f32,
    pub aspect_ratio: f32,
    pub transform: Transform,
    pub speed: f32,
    pub last_mouse_pos: Vec2,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            f_near: 0.1,
            f_far: 100.0,
            fov: 80.0 / 57.3,
            aspect_ratio: 1.0,
            transform: Transform::IDENTITY,
            speed: 0.1,
            last_mouse_pos: glam::vec2(0.0, 0.0),
        }
    }
}

impl Camera {
    pub fn projection(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov, self.aspect_ratio, self.f_near, self.f_far)
    }
    pub fn view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.transform.translation,
            self.transform.translation + self.transform.forward(),
            self.transform.up(),
        )
    }
    // Move with WASD and turn with holding mouse
    pub fn move_cam(&mut self, window: &Window) {
        let mut axis = glam::vec2(0.0, 0.0);
        if window.is_key_down(Key::A) {
            axis.x -= 1.0;
        }
        if window.is_key_down(Key::D) {
            axis.x += 1.0;
        }
        if window.is_key_down(Key::W) {
            axis.y += 1.0;
        }
        if window.is_key_down(Key::S) {
            axis.y -= 1.0;
        }
        let move_vec = self.transform.translation
            + self.transform.right() * self.speed * axis.x
            + self.transform.forward() * self.speed * axis.y;

        self.transform.set_translation(move_vec);

        window.get_mouse_pos(MouseMode::Clamp).map(|mouse| {
            if window.get_mouse_down(minifb::MouseButton::Left) {
                let dx = (0.25 * (mouse.0 - self.last_mouse_pos.x)) / 57.3;
                let dy = (0.25 * (mouse.1 - self.last_mouse_pos.y)) / 57.3;
                self.transform.set_rotation(
                    glam::Quat::from_axis_angle(self.transform.right(), dy)
                        * self.transform.rotation,
                );
                self.transform.set_rotation(
                    glam::Quat::from_axis_angle(glam::vec3(0.0, 1.0, 0.0), dx)
                        * self.transform.rotation,
                );
            }
            self.last_mouse_pos.x = mouse.0;
            self.last_mouse_pos.y = mouse.1;
        });
    }
}
