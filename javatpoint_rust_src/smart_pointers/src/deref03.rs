/*
cargo run -p smart_pointers --bin deref01
cargo fmt --verbose --package smart_pointers
*/

#[allow(dead_code)]
struct MyBox<T>(T);
impl<T> MyBox<T> {
    fn example(y: T) -> MyBox<T> {
        MyBox(y)
    }
}
fn main() {
    let a = 8;

    let box1 = Box::new(a);
    print!("Value of *box1 is {}", *box1);

    //    let b = MyBox::example(a);
    //    print!("Value of *b is {}",*b);
}
