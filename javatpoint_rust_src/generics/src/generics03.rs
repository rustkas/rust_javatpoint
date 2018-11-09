/*
cargo run -p generics --bin generics03
cargo fmt --verbose --package generics
*/
struct Value<T> {
    a: T,
    b: T,
}
fn main() {
    let integer = Value { a: 2, b: 3 };
    let float = Value { a: 7.8, b: 12.3 };
    println!("Integer values : {}, {}", integer.a, integer.b);
    println!("Float values :{}, {}", float.a, float.b);
}
