/*
cargo run -p smart_pointers --bin refcell03
cargo fmt --verbose --package smart_pointers
*/
use std::cell::RefCell;

fn main() {
    let a = RefCell::new(10);
    let b = a.borrow();
    // let c = a.borrow_mut(); // cause panic.
    println!("Value of b is : {}", b);
    // println!("Value of c is : {}", c);
}
