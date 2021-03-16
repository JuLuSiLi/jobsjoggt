pub mod vector_math;
use vector_math::*;
pub mod transform;
use transform::Transform;

fn main() {
    let mut t = Transform::new();
    t.set_position(Vector3::new(2.0, -5.0, 3.0));
    t.set_rotation(Vector3::new(0.0, 0.0, 90.0));
    t.set_scale(Vector3::new(2.0, 1.0, 1.0));

    let p = Vector3::new(1.0, 0.0, 0.0);

    println!("{:?}", t.transform_point(p));
}