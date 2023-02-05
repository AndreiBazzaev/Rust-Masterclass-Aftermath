use glam::{Mat4, Quat, Vec3};

#[derive(Debug, Copy, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    pub const IDENTITY: Self = Self {
        translation: Vec3::ZERO,
        rotation: Quat::IDENTITY,
        scale: Vec3::ONE,
    };

    pub fn new(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation: rotation.normalize(),
            scale,
        }
    }

    pub fn model_mat(&self) -> Mat4 {
        let model_mat = Mat4::from_translation(self.translation)
            * Mat4::from_quat(self.rotation)
            * Mat4::from_scale(self.scale);
        model_mat
    }
    pub fn translation(&self) -> Vec3 {
        self.translation
    }
    pub fn set_translation(&mut self, new_translation: Vec3) {
        self.translation = new_translation;
    }

    pub fn set_rotation(&mut self, new_rotation: Quat) {
        self.rotation = new_rotation.normalize();
    }
    pub fn rotation(&mut self) -> &Quat {
        &self.rotation
    }

    pub fn set_scale(&mut self, new_scale: Vec3) {
        self.scale = new_scale;
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * -Vec3::Z
    }
}
