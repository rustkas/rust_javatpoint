/*
cargo run -p control_statements --bin ifelse
cargo fmt --verbose --package control_statements
*/

fn main() {
    let a = 3;
    let b = 4;
    if a > b {
        println!("a is greater than b");
    } else {
        println!("a is smaller than b");
    }
}
