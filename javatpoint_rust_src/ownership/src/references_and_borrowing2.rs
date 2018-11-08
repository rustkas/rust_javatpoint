/*
cargo run -p ownership --bin references_and_borrowing2
cargo fmt --verbose --package ownership
*/
fn main() {
    let x = 1;
    value_changed(&x)
}
fn value_changed(y: &i32) {
    //*y = 9;
}
