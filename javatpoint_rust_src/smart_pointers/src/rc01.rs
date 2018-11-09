/*
cargo run -p smart_pointers --bin rc01
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let a = Cons(10, Box::new(Cons(15, Box::new(Nil))));
    let b = Cons(2, Box::new(a));
    let c = Cons(1, Box::new(a));

    println!("{:?}", &a);
    println!("{:?}", &b);
    println!("{:?}", &c);
}
