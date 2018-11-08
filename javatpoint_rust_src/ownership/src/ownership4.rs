/*
cargo run -p ownership --bin ownership4
cargo fmt --verbose --package ownership
*/
fn main() {
    let x = gives_ownership();
    println!("value of x is {}", x);
}

fn gives_ownership() -> u32 {
    let y = 100;
    y
}
