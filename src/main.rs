pub mod vector_math;
use vector_math::*;
pub mod transform;
use transform::Transform;

fn main() {
    let t = Transform::new();
    println!("{:?}", t);
}