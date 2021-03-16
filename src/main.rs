mod vector_math;
use vector_math::*;

fn main() {
    let m1 = Matrix4x4::perspective_projection_sym(10.0, 6.0, 2.0, 4.0);
    let v1 = Vector4::new(-10.0, -6.0, -4.0, 1.0);
    let v2 = &m1 * v1;
    println!("m1: {:?}", m1);
    println!("m1 * v1 (h): {:?}", v2 / v2.w);
}