#[derive(Debug)]
pub struct Matrix4x4 {
    r0: (f32, f32, f32, f32),
    r1: (f32, f32, f32, f32),
    r2: (f32, f32, f32, f32),
    r3: (f32, f32, f32, f32),
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            r0: (1.0, 0.0, 0.0, 0.0),
            r1: (0.0, 1.0, 0.0, 0.0),
            r2: (0.0, 0.0, 1.0, 0.0),
            r3: (0.0, 0.0, 0.0, 1.0),
        }
    }
}