/*
cargo run -p vectors --bin string5
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut s1 = String::from("Hello");
    let s2 = "World";
    s1.push_str(s2);
    print!("{}", s2);
}
