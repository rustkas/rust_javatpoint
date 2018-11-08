/*
cargo run -p control_statements --bin letif
cargo fmt --verbose --package control_statements
*/

fn main() {
    let a = if true { 1 } else { 2 };

    println!("value of a is: {}", a);
}
