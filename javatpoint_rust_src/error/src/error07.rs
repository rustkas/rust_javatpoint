/*
cargo run -p error --bin error7
cargo fmt --verbose --package error
*/

use std::fs::File;

fn main() {
    let _f: u32 = File::open("vector.txt");
}
