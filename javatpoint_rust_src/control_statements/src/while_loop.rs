/*
cargo run -p control_statements --bin while_loop
cargo fmt --verbose --package control_statements
*/

fn main() {
    let mut i = 1;
    while i <= 10 {
        print!("{}", i);
        print!(" ");
        i = i + 1;
    }
}
