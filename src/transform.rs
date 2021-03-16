use crate::vector_math::*;

#[derive(Debug)]
pub struct Transform {
    position: Vector3,
    rotation: Vector3, // In Degrees
    scale: Vector3,

    has_changed: bool,

    local_to_world: Matrix4x4,
    world_to_local: Matrix4x4,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),

            has_changed: false,

            local_to_world: Matrix4x4::identity(),
            world_to_local: Matrix4x4::identity(),
        }
    }
    
    pub fn set_position(&mut self, position: Vector3) {
        self.position = position;
        self.has_changed = true;
    }

    pub fn get_position(&self) -> Vector3 {
        self.position
    }

    pub fn set_rotation(&mut self, rotation: Vector3) {
        self.rotation = rotation;
        self.has_changed = true;
    }

    pub fn get_rotation(&self) -> Vector3 {
        self.rotation
    }

    pub fn set_scale(&mut self, scale: Vector3) {
        self.scale = scale;
        self.has_changed = true;
    }

    pub fn get_scale(&self) -> Vector3 {
        self.scale
    }

    // World-space to local-space
    pub fn transform_point(&mut self, point: Vector3) -> Vector3 {
        if self.has_changed {
            self.compute_matrices();
        }
        
        self.local_to_world * point
    }

    pub fn inv_transform_point(&mut self, point: Vector3) -> Vector3 {
        if self.has_changed {
            self.compute_matrices();
        }
        
        self.world_to_local * point
    }

    // Apply order:
    // Scale
    // Rotation: Z-X-Y
    // Translation
    fn compute_matrices(&mut self) {
        self.local_to_world =
            Matrix4x4::translation(self.position) *
            (Matrix4x4::y_rotation(self.rotation.y.to_radians()) *
            (Matrix4x4::x_rotation(self.rotation.x.to_radians()) *
            (Matrix4x4::z_rotation(self.rotation.z.to_radians()) *
            Matrix4x4::scale(self.scale))));
        
        self.world_to_local =
            Matrix4x4::scale(self.scale) *
            (Matrix4x4::z_rotation(self.rotation.z.to_radians()) *
            (Matrix4x4::x_rotation(self.rotation.x.to_radians()) *
            (Matrix4x4::y_rotation(self.rotation.y.to_radians()) *
            Matrix4x4::translation(self.position))));
    }
}