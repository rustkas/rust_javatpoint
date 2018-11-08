/*
cargo run -p vectors --bin string7
cargo fmt --verbose --package vectors
*/

fn main() {
    let s1 = String::from("javaTpoint ");
    let s2 = String::from("tutorial!!");
    let s3 = s1 + &s2;
    print!("{}", s3);
}
