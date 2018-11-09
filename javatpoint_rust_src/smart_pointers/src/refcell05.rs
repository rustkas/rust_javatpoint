/*
cargo run -p smart_pointers --bin refcell05
cargo fmt --verbose --package smart_pointers
*/

// Let's see a simple example of panic condition:

use std::cell::RefCell;
fn main() {
    let a = RefCell::new(15);
    println!("{:?}", &a);

    let b = a.borrow_mut();
    let c = a.borrow_mut();

    println!("{:?}", &b);
    println!("{:?}", &c);
}
