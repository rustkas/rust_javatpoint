/*
cargo run -p generics --bin generics04
cargo fmt --verbose --package generics
*/
struct Value<T> {
    a: T,
    b: T,
}
fn main() {
    //    let c = Value{a:2, b:3.6};
    //    println!("c values : {},{}",c.a,c.b);

    let c = Value { a: 2, b: 3 };
    println!("c values : {}, {}", c.a, c.b);
}
