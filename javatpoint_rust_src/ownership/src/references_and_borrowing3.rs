/*
cargo run -p ownership --bin references_and_borrowing3
cargo fmt --verbose --package ownership
*/
fn main() {
    let mut x = 1;
    value_changed(&mut x);
    println!("After modifying, the value of x is {}", x);
}
fn value_changed(y: &mut i32) {
    *y = 9;
}
