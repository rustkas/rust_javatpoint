/*
cargo run -p error --bin error9
cargo fmt --verbose --package error
*/

use std::fs::File;

fn main() {
    File::open("hello.txt").unwrap();
}
