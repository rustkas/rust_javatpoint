/*
cargo run -p vectors --bin string5
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut s = String::from("java");
    s.push('c');
    print!("{}", s);
}
