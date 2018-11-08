/*
cargo run -p vectors --bin string4
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut s = String::from("java is a");
    s.push_str(" programming language");
    print!("{}", s);
}
