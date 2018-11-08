/*
cargo run -p structs --bin struct2
cargo fmt --verbose --package structs
*/

struct Triangle {
    base: f64,
    height: f64,
}

fn main() {
    let triangle = Triangle {
        base: 20.0,
        height: 30.0,
    };
    print!("Area of a right angled triangle is {}", area(&triangle));
}

fn area(t: &Triangle) -> f64 {
    0.5 * t.base * t.height
}
