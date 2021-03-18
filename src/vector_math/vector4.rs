use std::ops;

#[derive(Debug, Copy, Clone)]
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

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalized(self) -> Vector4 {
        self / self.length()
    }
}

// Vector + Vector
impl<'a, 'b> ops::Add<Vector4> for Vector4 {
    type Output = Vector4;

    fn add(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

// Vector +=
impl<'a, 'b> ops::AddAssign<Vector4> for Vector4 {
    fn add_assign(&mut self, other: Vector4) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
    }
}

// Vector - Vector
impl<'a, 'b> ops::Sub<Vector4> for Vector4 {
    type Output = Vector4;

    fn sub(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

// Vector -=
impl<'a, 'b> ops::SubAssign<Vector4> for Vector4 {
    fn sub_assign(&mut self, other: Vector4) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
    }
}

// Vector * Vector (Dot product)
impl<'a, 'b> ops::Mul<Vector4> for Vector4 {
    type Output = f32;

    fn mul(self, other: Vector4) -> f32 {
        self.x * other.x +
        self.y * other.y +
        self.z * other.z +
        self.w * other.w
    }
}

// Vector * Scalar
impl<'a, 'b> ops::Mul<f32> for Vector4 {
    type Output = Vector4;

    fn mul(self, other: f32) -> Vector4 {
        Vector4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

// Scalar * Vector
impl<'a, 'b> ops::Mul<Vector4> for f32 {
    type Output = Vector4;

    fn mul(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self * other.x,
            y: self * other.y,
            z: self * other.z,
            w: self * other.w,
        }
    }
}

// Vector *=
impl<'a, 'b> ops::MulAssign<f32> for Vector4 {
    fn mul_assign(&mut self, other: f32) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
        self.w *= other;
    }
}

// Vector / Scalar
impl<'a, 'b> ops::Div<f32> for Vector4 {
    type Output = Vector4;

    fn div(self, other: f32) -> Vector4 {
        let other = 1.0 / other;
        Vector4 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
            w: self.w * other,
        }
    }
}

// Scalar / Vector
impl<'a, 'b> ops::Div<Vector4> for f32 {
    type Output = Vector4;

    fn div(self, other: Vector4) -> Vector4 {
        Vector4 {
            x: self / other.x,
            y: self / other.y,
            z: self / other.z,
            w: self / other.w,
        }
    }
}

// Vector /=
impl<'a, 'b> ops::DivAssign<f32> for Vector4 {
    fn div_assign(&mut self, other: f32) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
        self.w /= other;
    }
}

// -Vector
impl<'a> ops::Neg<> for Vector4 {
    type Output = Vector4;

    fn neg(self) -> Vector4 {
        Vector4 {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}