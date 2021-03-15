mod matrix_4x4_math;
use matrix_4x4_math::Matrix4x4;

fn main() {
    let m = Matrix4x4::identity();
    println!("m: {:?}", m);
}