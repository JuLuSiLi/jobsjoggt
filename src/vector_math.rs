use std::ops;

#[derive(Debug)]
pub struct Vector4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Vector4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Vector4 {
        Vector4 {
            x: x,
            y: y,
            z: z,
            w: w,
        }
    }
}

impl<'a, 'b> ops::Add<&'b Vector4> for &'a Vector4 {
    type Output = Vector4;

    fn add(self, other: &'b Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Vector4> for &'a Vector4 {
    type Output = Vector4;

    fn sub(self, other: &'b Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Vector4> for &'a Vector4 {
    type Output = f32;

    fn mul(self, other: &'b Vector4) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }
}