/*
cargo run -p control_statements --bin for_loop2
cargo fmt --verbose --package control_statements
*/

fn main() {
    let mut result;
    for i in 1..11 {
        result = 2 * i;
        println!("2 * {} = {}", i, result);
    }
}
