use std::ops;
use crate::vector_math::*;

#[derive(Debug)]
pub struct Matrix4x4 {
    pub r0: Vector4,
    pub r1: Vector4,
    pub r2: Vector4,
    pub r3: Vector4,
}

impl Matrix4x4 {
    pub fn identity() -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(1.0, 0.0, 0.0, 0.0),
            r1: Vector4::new(0.0, 1.0, 0.0, 0.0),
            r2: Vector4::new(0.0, 0.0, 1.0, 0.0),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn scale(s: &Vector3) -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(s.x, 0.0, 0.0, 0.0),
            r1: Vector4::new(0.0, s.y, 0.0, 0.0),
            r2: Vector4::new(0.0, 0.0, s.z, 0.0),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }
    
    pub fn translation(t: &Vector3) -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(1.0, 0.0, 0.0, t.x),
            r1: Vector4::new(0.0, 1.0, 0.0, t.y),
            r2: Vector4::new(0.0, 0.0, 1.0, t.z),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    // Rotations in radians
    pub fn x_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(1.0, 0.0, 0.0, 0.0),
            r1: Vector4::new(0.0, r.cos(), -r.sin(), 0.0),
            r2: Vector4::new(0.0, r.sin(), r.cos(), 0.0),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn y_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(r.cos(), 0.0, r.sin(), 0.0),
            r1: Vector4::new(0.0, 1.0, 0.0, 0.0),
            r2: Vector4::new(-r.sin(), 0.0, r.cos(), 0.0),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn z_rotation(r: f32) -> Matrix4x4 {
        Matrix4x4 {
            r0: Vector4::new(r.cos(), -r.sin(), 0.0, 0.0),
            r1: Vector4::new(r.sin(), r.cos(), 0.0, 0.0),
            r2: Vector4::new(0.0, 0.0, 1.0, 0.0),
            r3: Vector4::new(0.0, 0.0, 0.0, 1.0),
        }
    }

    pub fn c0(&self) -> Vector4 {
        Vector4::new(self.r0.x, self.r1.x, self.r2.x, self.r3.x)
    }

    pub fn c1(&self) -> Vector4 {
        Vector4::new(self.r0.y, self.r1.y, self.r2.y, self.r3.y)
    }

    pub fn c2(&self) -> Vector4 {
        Vector4::new(self.r0.z, self.r1.z, self.r2.z, self.r3.z)
    }

    pub fn c3(&self) -> Vector4 {
        Vector4::new(self.r0.w, self.r1.w, self.r2.w, self.r3.w)
    }
}

impl<'a, 'b> ops::Add<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn add(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            r0: &self.r0 + &other.r0,
            r1: &self.r1 + &other.r1,
            r2: &self.r2 + &other.r2,
            r3: &self.r3 + &other.r3,
        }
    }
}

impl<'a, 'b> ops::Sub<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn sub(self, other: &'b Matrix4x4) -> Matrix4x4 {
        Matrix4x4 {
            r0: &self.r0 - &other.r0,
            r1: &self.r1 - &other.r1,
            r2: &self.r2 - &other.r2,
            r3: &self.r3 - &other.r3,
        }
    }
}

impl<'a, 'b> ops::Mul<&'b Matrix4x4> for &'a Matrix4x4 {
    type Output = Matrix4x4;

    fn mul(self, other: &'b Matrix4x4) -> Matrix4x4 {
        let other_c0 = other.c0();
        let other_c1 = other.c1();
        let other_c2 = other.c2();
        let other_c3 = other.c3();
        
        Matrix4x4 {
            r0: Vector4::new(&self.r0 * &other_c0, &self.r0 * &other_c1, &self.r0 * &other_c2, &self.r0 * &other_c3),
            r1: Vector4::new(&self.r1 * &other_c0, &self.r1 * &other_c1, &self.r1 * &other_c2, &self.r1 * &other_c3),
            r2: Vector4::new(&self.r2 * &other_c0, &self.r2 * &other_c1, &self.r2 * &other_c2, &self.r2 * &other_c3),
            r3: Vector4::new(&self.r3 * &other_c0, &self.r3 * &other_c1, &self.r3 * &other_c2, &self.r3 * &other_c3),
        }
    }
}