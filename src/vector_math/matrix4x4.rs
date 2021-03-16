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

// Constructors
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

    pub fn scale(s: Vector3) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                s.x, 0.0, 0.0, 0.0,
                0.0, s.y, 0.0, 0.0,
                0.0, 0.0, s.z, 0.0,
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }
    
    pub fn translation(t: Vector3) -> Matrix4x4 {
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

    pub fn orthographic_projection_sym(width: f32, height: f32, near: f32, far: f32) -> Matrix4x4 {
        let right = width * 0.5;
        let top = height * 0.5;
        Matrix4x4 {
            m: [
                1.0 / right, 0.0, 0.0, 0.0,
                0.0, 1.0 / top, 0.0, 0.0,
                0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn orthographic_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                2.0 / (right - left), 0.0, 0.0, -(right + left) / (right - left),
                0.0, 2.0 / (top - bottom), 0.0, -(top + bottom) / (top - bottom),
                0.0, 0.0, -2.0 / (far - near), -(far + near) / (far - near),
                0.0, 0.0, 0.0, 1.0
            ]
        }
    }

    pub fn perspective_projection_sym(width: f32, height: f32, near: f32, far: f32) -> Matrix4x4 {
        let right = width * 0.5;
        let top = height * 0.5;
        Matrix4x4 {
            m: [
                near / right, 0.0, 0.0, 0.0,
                0.0, near / top, 0.0, 0.0,
                0.0, 0.0, -(far + near) / (far - near), -2.0 * far * near / (far - near),
                0.0, 0.0, -1.0, 0.0
            ]
        }
    }

    pub fn perspective_projection(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Matrix4x4 {
        Matrix4x4 {
            m: [
                2.0 * near / (right - left), 0.0, (right + left) / (right - left), 0.0,
                0.0, 2.0 * near / (top - bottom), (top + bottom) / (top - bottom), 0.0,
                0.0, 0.0, -(far + near) / (far - near), -2.0 * far * near / (far - near),
                0.0, 0.0, -1.0, 0.0
            ]
        }
    }
}

impl Matrix4x4 {
    pub fn inverse(&self) -> Matrix4x4 {
        let a2323 = self.m[10] * self.m[15] - self.m[11] * self.m[14];
        let a1323 = self.m[09] * self.m[15] - self.m[11] * self.m[13];
        let a1223 = self.m[09] * self.m[14] - self.m[10] * self.m[13];
        let a0323 = self.m[08] * self.m[15] - self.m[11] * self.m[12];
        let a0223 = self.m[08] * self.m[14] - self.m[10] * self.m[12];
        let a0123 = self.m[08] * self.m[13] - self.m[09] * self.m[12];
        let a2313 = self.m[06] * self.m[15] - self.m[07] * self.m[14];
        let a1313 = self.m[05] * self.m[15] - self.m[07] * self.m[13];
        let a1213 = self.m[05] * self.m[14] - self.m[06] * self.m[13];
        let a2312 = self.m[06] * self.m[11] - self.m[07] * self.m[10];
        let a1312 = self.m[05] * self.m[11] - self.m[07] * self.m[09];
        let a1212 = self.m[05] * self.m[10] - self.m[06] * self.m[09];
        let a0313 = self.m[04] * self.m[15] - self.m[07] * self.m[12];
        let a0213 = self.m[04] * self.m[14] - self.m[06] * self.m[12];
        let a0312 = self.m[04] * self.m[11] - self.m[07] * self.m[08];
        let a0212 = self.m[04] * self.m[10] - self.m[06] * self.m[08];
        let a0113 = self.m[04] * self.m[13] - self.m[05] * self.m[12];
        let a0112 = self.m[04] * self.m[09] - self.m[05] * self.m[08];

        let det =
             self.m[00] * (self.m[05] * a2323 - self.m[06] * a1323 + self.m[07] * a1223) 
            -self.m[01] * (self.m[04] * a2323 - self.m[06] * a0323 + self.m[07] * a0223) 
            +self.m[02] * (self.m[04] * a1323 - self.m[05] * a0323 + self.m[07] * a0123) 
            -self.m[03] * (self.m[04] * a1223 - self.m[05] * a0223 + self.m[06] * a0123);
        let det = 1.0 / det;

        Matrix4x4 {
            m: [
                det *  (self.m[05] * a2323 - self.m[06] * a1323 + self.m[07] * a1223),
                det * -(self.m[01] * a2323 - self.m[02] * a1323 + self.m[03] * a1223),
                det *  (self.m[01] * a2313 - self.m[02] * a1313 + self.m[03] * a1213),
                det * -(self.m[01] * a2312 - self.m[02] * a1312 + self.m[03] * a1212),
                det * -(self.m[04] * a2323 - self.m[06] * a0323 + self.m[07] * a0223),
                det *  (self.m[00] * a2323 - self.m[02] * a0323 + self.m[03] * a0223),
                det * -(self.m[00] * a2313 - self.m[02] * a0313 + self.m[03] * a0213),
                det *  (self.m[00] * a2312 - self.m[02] * a0312 + self.m[03] * a0212),
                det *  (self.m[04] * a1323 - self.m[05] * a0323 + self.m[07] * a0123),
                det * -(self.m[00] * a1323 - self.m[01] * a0323 + self.m[03] * a0123),
                det *  (self.m[00] * a1313 - self.m[01] * a0313 + self.m[03] * a0113),
                det * -(self.m[00] * a1312 - self.m[01] * a0312 + self.m[03] * a0112),
                det * -(self.m[04] * a1223 - self.m[05] * a0223 + self.m[06] * a0123),
                det *  (self.m[00] * a1223 - self.m[01] * a0223 + self.m[02] * a0123),
                det * -(self.m[00] * a1213 - self.m[01] * a0213 + self.m[02] * a0113),
                det *  (self.m[00] * a1212 - self.m[01] * a0212 + self.m[02] * a0112)
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
impl<'a, 'b> ops::Mul<Vector4> for &'a Matrix4x4 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.m[00] * other.x + self.m[01] * other.y + self.m[02] * other.z + self.m[03] * other.w,
            y: self.m[04] * other.x + self.m[05] * other.y + self.m[06] * other.z + self.m[07] * other.w,
            z: self.m[08] * other.x + self.m[09] * other.y + self.m[10] * other.z + self.m[11] * other.w,
            w: self.m[12] * other.x + self.m[13] * other.y + self.m[14] * other.z + self.m[15] * other.w,
        }
    }
}