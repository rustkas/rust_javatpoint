/*
cargo run -p smart_pointers --bin rc02
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}
use std::rc::Rc;
use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(10, Rc::new(Cons(15, Rc::new(Nil)))));
    let b = Cons(2, Rc::clone(&a));
    let c = Cons(1, Rc::clone(&a));

    println!("{:?}", &a);
    println!("{:?}", &b);
    println!("{:?}", &c);
}
