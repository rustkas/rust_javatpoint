/*
cargo run -p structs --bin struct3
cargo fmt --verbose --package structs
*/

struct Square {
    a: u32,
}
impl Square {
    fn area(&self) -> u32 {
        self.a * self.a
    }
}

fn main() {
    let square = Square { a: 10 };
    print!("Area of square is {}", square.area());
}
