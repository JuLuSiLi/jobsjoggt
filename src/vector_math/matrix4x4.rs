use std::ops;
use crate::vector_math::*;

// Row by row
// | 0  1  2  3  |
// | 4  5  6  7  |
// | 8  9  10 11 |
// | 12 13 14 15 |
#[derive(Debug)]
pub struct Matrix4x4 {
    pub m: [f32; 16]
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            m: [
                1.0, 0.0, 0.0, 0.0,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn scale(s: &Vector3) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                s.x, 0.0, 0.0, 0.0,
                0.0, s.y, 0.0, 0.0,
                0.0, 0.0, s.z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn translation(t: &Vector3) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                1.0, 0.0, 0.0, t.x,
                0.0, 1.0, 0.0, t.y,
                0.0, 0.0, 1.0, t.z,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    // Rotations in radians
    pub fn x_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                1.0, 0.0, 0.0, 0.0,
                0.0, r.cos(), -r.sin(), 0.0,
                0.0, r.sin(), r.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn y_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                r.cos(), 0.0, r.sin(), 0.0,
                0.0, 1.0, 0.0, 0.0,
                -r.sin(), 0.0, r.cos(), 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn z_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                r.cos(), -r.sin(), 0.0, 0.0,
                r.sin(), r.cos(), 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
}

// Matrix + Matrix
impl<'a, 'b> ops::Add<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                self.m[00] + other.m[00], self.m[01] + other.m[01], self.m[02] + other.m[02], self.m[03] + other.m[03],
                self.m[04] + other.m[04], self.m[05] + other.m[05], self.m[06] + other.m[06], self.m[07] + other.m[07],
                self.m[08] + other.m[08], self.m[09] + other.m[09], self.m[10] + other.m[10], self.m[11] + other.m[11],
                self.m[12] + other.m[12], self.m[13] + other.m[13], self.m[14] + other.m[14], self.m[15] + other.m[15]
            ]
        }
    }
}

// Matrix - Matrix
impl<'a, 'b> ops::Sub<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                self.m[00] - other.m[00], self.m[01] - other.m[01], self.m[02] - other.m[02], self.m[03] - other.m[03],
                self.m[04] - other.m[04], self.m[05] - other.m[05], self.m[06] - other.m[06], self.m[07] - other.m[07],
                self.m[08] - other.m[08], self.m[09] - other.m[09], self.m[10] - other.m[10], self.m[11] - other.m[11],
                self.m[12] - other.m[12], self.m[13] - other.m[13], self.m[14] - other.m[14], self.m[15] - other.m[15]
            ]
        }
    }
}

// Matrix * Matrix
impl<'a, 'b> ops::Mul<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                self.m[00] * other.m[00] + self.m[01] * other.m[04] + self.m[02] * other.m[08] + self.m[03] * other.m[12],
                self.m[00] * other.m[01] + self.m[01] * other.m[05] + self.m[02] * other.m[09] + self.m[03] * other.m[13],
                self.m[00] * other.m[02] + self.m[01] * other.m[06] + self.m[02] * other.m[10] + self.m[03] * other.m[14],
                self.m[00] * other.m[03] + self.m[01] * other.m[07] + self.m[02] * other.m[11] + self.m[03] * other.m[15],

                self.m[04] * other.m[00] + self.m[05] * other.m[04] + self.m[06] * other.m[08] + self.m[07] * other.m[12],
                self.m[04] * other.m[01] + self.m[05] * other.m[05] + self.m[06] * other.m[09] + self.m[07] * other.m[13],
                self.m[04] * other.m[02] + self.m[05] * other.m[06] + self.m[06] * other.m[10] + self.m[07] * other.m[14],
                self.m[04] * other.m[03] + self.m[05] * other.m[07] + self.m[06] * other.m[11] + self.m[07] * other.m[15],

                self.m[08] * other.m[00] + self.m[09] * other.m[04] + self.m[10] * other.m[08] + self.m[11] * other.m[12],
                self.m[08] * other.m[01] + self.m[09] * other.m[05] + self.m[10] * other.m[09] + self.m[11] * other.m[13],
                self.m[08] * other.m[02] + self.m[09] * other.m[06] + self.m[10] * other.m[10] + self.m[11] * other.m[14],
                self.m[08] * other.m[03] + self.m[09] * other.m[07] + self.m[10] * other.m[11] + self.m[11] * other.m[15],

                self.m[12] * other.m[00] + self.m[13] * other.m[04] + self.m[14] * other.m[08] + self.m[15] * other.m[12],
                self.m[12] * other.m[01] + self.m[13] * other.m[05] + self.m[14] * other.m[09] + self.m[15] * other.m[13],
                self.m[12] * other.m[02] + self.m[13] * other.m[06] + self.m[14] * other.m[10] + self.m[15] * other.m[14],
                self.m[12] * other.m[03] + self.m[13] * other.m[07] + self.m[14] * other.m[11] + self.m[15] * other.m[15],
            ]
        }
    }
}

// Matrix * Scalar
impl<'a, 'b> ops::Mul<f32> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                self.m[00] * other, self.m[01] * other, self.m[02] * other, self.m[03] * other,
                self.m[04] * other, self.m[05] * other, self.m[06] * other, self.m[07] * other,
                self.m[08] * other, self.m[09] * other, self.m[10] * other, self.m[11] * other,
                self.m[12] * other, self.m[13] * other, self.m[14] * other, self.m[15] * other
            ]
        }
    }
}

// Matrix * Vector
impl<'a, 'b> ops::Mul<&'b Vector4> for &'a Matrix4x4 {
    type Output = Vector4;

    fn mul(self, other: &'b Vector4) -> Vector4 {
        Vector4 {
            x: self.m[00] * other.x + self.m[01] * other.y + self.m[02] * other.z + self.m[03] * other.w,
            y: self.m[04] * other.x + self.m[05] * other.y + self.m[06] * other.z + self.m[07] * other.w,
            z: self.m[08] * other.x + self.m[09] * other.y + self.m[10] * other.z + self.m[11] * other.w,
            w: self.m[12] * other.x + self.m[13] * other.y + self.m[14] * other.z + self.m[15] * other.w,
        }
    }
}