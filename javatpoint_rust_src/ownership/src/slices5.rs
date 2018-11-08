/*
cargo run -p ownership --bin slices5
cargo fmt --verbose --package ownership
*/
fn main() {
    let arr = [100, 200, 300, 400, 500]; // array initialization
    let a = &arr[1..=3]; // retrieving second,third and fourth element
    println!("{:?}", a);
}
