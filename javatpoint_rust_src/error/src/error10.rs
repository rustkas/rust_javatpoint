/*
cargo run -p error --bin error10
cargo fmt --verbose --package error
*/

use std::fs::File;

fn main() {
    File::open("hello.txt").expect("Not able to find the file hello.txt");
}
