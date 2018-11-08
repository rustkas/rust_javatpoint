/*
cargo run -p module --bin use2
cargo fmt --verbose --package module
*/
#[derive(Debug)]
enum Flagcolor {
    Orange,
    White,
    Green,
}
use Flagcolor::{Green, Orange, White};
fn main() {
    let o = Orange;
    let w = White;
    let g = Green;
    println!("{:?}", o);
    println!("{:?}", w);
    println!("{:?}", g);
}
