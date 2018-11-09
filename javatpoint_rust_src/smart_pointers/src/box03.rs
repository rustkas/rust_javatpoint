/*
cargo run -p smart_pointers --bin box01
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use List::{Cons, Nil};
fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    print!("{:?}", list);
    //    for i in list.iter() {
    //        print!("{:?}", i);
    //    }
}
