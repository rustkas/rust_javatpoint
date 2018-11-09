/*
cargo run -p smart_pointers --bin box02
cargo fmt --verbose --package smart_pointers
*/

enum List {
    cons(i32, List),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    let list = List::Cons(1, Cons(2, Cons(3, Nil)));
    for i in list.iter() {
        print!("{:?}", i);
    }
}
