/*
cargo run -p control_statements --bin letif2
cargo fmt --verbose --package control_statements
*/

fn main() {
    let a = if false { 9 } else { "javaTpoint" };

    println!("value of a is: {}", a);
}
