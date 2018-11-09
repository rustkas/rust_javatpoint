/*
cargo run -p error --bin error8
cargo fmt --verbose --package error
*/

use std::fs::File;
fn main() {
    let f = File::open("vector.txt");
    match f {
        Ok(file) => file,
        Err(error) => panic!("There was a problem opening the file: {:?}", error),
    };
}
