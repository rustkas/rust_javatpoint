/*
cargo run -p smart_pointers --bin deref04
cargo fmt --verbose --package smart_pointers
*/

struct MyBox<T> {
    a: T,
}
use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.a
    }
}
fn main() {
    let b = MyBox { a: 10 };
    println!("{}", *(b.deref()));
    println!("{}", *b);
}
