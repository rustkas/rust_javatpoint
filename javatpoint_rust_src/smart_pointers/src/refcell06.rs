/*
cargo run -p smart_pointers --bin refcell06
cargo fmt --verbose --package smart_pointers
*/

// Let's see a simple example of panic condition:

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<String>>, Rc<List>),
    Nil,
}
use std::cell::RefCell;
use std::rc::Rc;
use List::{Cons, Nil};
fn main() {
    let val = Rc::new(RefCell::new(String::from("java")));
    let a = Rc::new(Cons(Rc::clone(&val), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(String::from("C"))), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(String::from("C++"))), Rc::clone(&a));
    *val.borrow_mut() = String::from("C# language");
    println!("value of a is : {:?}", a);
    println!("value of b is : {:?}", b);
    println!("value of c is : {:?}", c);
}
