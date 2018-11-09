/*
cargo run -p smart_pointers --bin deref01
cargo fmt --verbose --package smart_pointers
*/

fn main() {
    let a = 11;
    let b = Box::new(a);
    print!("Value of *b is {}", *b);
}
