/*
cargo run -p error --bin error12
cargo fmt --verbose --package error
*/

use std::fs::File;
use std::io;
use std::io::Read;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("a.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    let a = read_username_from_file();
    print!("{:?}", a);
}
