/*
cargo run -p smart_pointers --bin refcell04
cargo fmt --verbose --package smart_pointers
*/
use std::cell::RefCell;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Sub;

fn main() {
    let a = RefCell::new(15);
    let b = a.borrow_mut();
    println!("Now, value of b is {}", b);

    println!("b + 1 = {}", b.add(1));
    println!("b - 1 = {}", b.sub(1));
    println!("b * 2 = {}", b.mul(2));
}
