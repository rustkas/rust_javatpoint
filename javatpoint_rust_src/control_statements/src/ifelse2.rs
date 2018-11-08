/*
cargo run -p control_statements --bin ifelse2
cargo fmt --verbose --package control_statements
*/

fn main() {
    let num = -5;
    if num > 0 {
        println!("number is greater than 0");
    } else if num < 0 {
        println!("number is less than 0 ");
    } else {
        println!("number is not equal to 0");
    }
}
