/*
cargo run -p control_statements --bin ifelse_nested
cargo fmt --verbose --package control_statements
*/

fn main() {
    let a = 5;
    let b = 6;
    if a != b {
        if a > b {
            println!("a is greater than b");
        } else {
            println!("a is less than b");
        }
    } else {
        println!("a is equal to b");
    }
}
