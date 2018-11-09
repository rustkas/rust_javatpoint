/*
cargo run -p generics --bin lifetime01
cargo fmt --verbose --package generics
*/

fn main() {
    let b;
    let a;

    b = 10;
    a = &b;

    println!("a : {}", a);
}
