fn main() {
    let a = 5;
    let b = 12;
    println!("Hello, jobsjoggt! Your result is: {}", add(a, b));
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }