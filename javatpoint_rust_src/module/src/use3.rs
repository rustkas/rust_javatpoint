/*
cargo run -p module --bin use3
cargo fmt --verbose --package module
*/
#[derive(Debug)]
enum Color {
    Red,
    Yellow,
    Green,
    Orange,
}

use Color::*;
fn main() {
    let red = Red;
    let yellow = Yellow;
    let green = Green;
    let orange = Orange;
    println!("{:?}", red);
    println!("{:?}", yellow);
    println!("{:?}", green);
    println!("{:?}", orange);
}
