use crate::vector_math::*;

#[derive(Debug)]
pub struct Transform {
    pub position: Vector3,
    pub rotation: Vector3,
    pub scale: Vector3,
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            position: Vector3::new(0.0, 0.0, 0.0),
            rotation: Vector3::new(0.0, 0.0, 0.0),
            scale: Vector3::new(1.0, 1.0, 1.0),
        }
    }
}