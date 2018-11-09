/*
cargo run -p smart_pointers --bin deref05
cargo fmt --verbose --package smart_pointers
*/

struct MyBox<T>(T);
use std::ops::Deref;
use std::ops::DerefMut;
impl<T> MyBox<T> {
    fn hello(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &T {
        &self.0
    }
}
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut T {
        &mut self.0
    }
}
fn print(m: &i32) {
    println!("{}", m);
}

fn add_one(m: &mut i32) {
    *m = *m + 1;
}
fn main() {
    let mut b = MyBox::hello(5);

    print(&b);
    add_one(&mut b);
    print(&b);
}
