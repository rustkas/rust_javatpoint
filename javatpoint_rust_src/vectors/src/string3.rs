/*
cargo run -p vectors --bin string3
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut s1 = "Hello, ".to_string();
    let s2 = "Rust!";
    s1.push_str(s2);
    print!("{}", s1);
}
