/*
cargo run -p generics --bin generics05
cargo fmt --verbose --package generics
*/
struct Program<T> {
    a: T,
    b: T,
}
impl<T> Program<T> {
    fn a(&self) -> &T {
        &self.a
    }

    fn b(&self) -> &T {
        &self.b
    }
}
fn main() {
    let p = Program { a: 5, b: 10 };

    println!("p.a() is {}. p.b() is {}.", p.a(), p.b());
}
