/*
cargo run -p smart_pointers --bin rc03
cargo fmt --verbose --package smart_pointers
*/

use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};
fn main() {
    let a = Rc::new(Cons(10, Rc::new(Cons(15, Rc::new(Nil)))));
    println!(
        "Reference count after creating a List : {}",
        Rc::strong_count(&a)
    );
    {
        let b = Cons(2, Rc::clone(&a));
        println!(
            "Reference count after creating b List : {}",
            Rc::strong_count(&a)
        );
        {
            let c = Cons(1, Rc::clone(&a));
            println!(
                "Reference count after creating c List : {}",
                Rc::strong_count(&a)
            );
            println!("c  = {:?}", &c);
        }

        println!("b = {:?}", &b);
    }

    drop(&a);
    println!("a = {:?}", &a);

    println!(
        "Reference count 'a' when 'c' and 'b' goes out of the scope : {}",
        Rc::strong_count(&a)
    );
}
