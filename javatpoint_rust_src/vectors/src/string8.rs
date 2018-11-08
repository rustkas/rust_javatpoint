/*
cargo run -p vectors --bin string8
cargo fmt --verbose --package vectors
*/

fn main() {
    let s1 = String::from("C");
    let s2 = String::from("is");
    let s3 = String::from("a");
    let s4 = String::from("programming");
    let s5 = String::from("language.");
    let s = format!("{} {} {} {} {}", s1, s2, s3, s4, s5);
    print!("{}", s);
}
