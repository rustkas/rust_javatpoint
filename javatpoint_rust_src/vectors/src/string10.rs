/*
cargo run -p vectors --bin string10
cargo fmt --verbose --package vectors
*/

fn main() {
    let s = "Hello World";
    let a = &s[1..4];
    print!("{}", a);
}
