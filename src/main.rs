mod vector_math;
use vector_math::*;

fn main() {
    let m1 = Matrix4x4 {
        m: [
            5.0, -1.0, 2.0, 8.0,
            2.0, 6.0, 2.0, 9.0,
            3.0, -4.0, 3.0, 1.0,
            1.0, -7.0, 4.0, -3.0
        ]
    };
    let m2 = Matrix4x4 {
        m: [
            2.0, 5.0, -1.0, -5.0,
            -2.0, -2.0, 6.0, 8.0,
            8.0, 7.0, 5.0, 3.0,
            9.0, -2.0, 1.0, 2.0
        ]
    };
    let m3 = &m1 * &m2;
    println!("m1: {:?}", m1);
    println!("m2: {:?}", m2);
    println!("m3: {:?}", m3);
    println!("m3^-1: {:?}", m3.inverse());
}