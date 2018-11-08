/*
cargo run -p ownership --bin ownership1
cargo fmt --verbose --package ownership
*/
fn main() {
    let x = 10;
    let _y = x;
    println!("value of x :{}", x);
}
