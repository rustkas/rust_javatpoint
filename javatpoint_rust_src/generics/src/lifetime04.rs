/*
cargo run -p generics --bin lifetime01
cargo fmt --verbose --package generics
*/

struct Example<'a> {
    x: &'a i32,
}

//struct Example {
//    x: &i32,
//}

fn main() {
    let y = &9;
    let b = Example { x: y };
    println!("{}", b.x);
}
