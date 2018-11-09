/*
cargo run -p generics --bin trait04
cargo fmt --verbose --package generics
*/

// Trait limitation
// If the trait is not defined in your scope,
// then it cannot be implemented on any data type.

use std::{fs::File, io::Write};

fn main() {
    let mut f = File::create("hello.txt").unwrap();
    let str = b"javaTpoint";
    let result = f.write(str).unwrap();
    println!("{}", result)
}
