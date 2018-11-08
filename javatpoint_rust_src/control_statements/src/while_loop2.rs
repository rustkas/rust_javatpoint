/*
cargo run -p control_statements --bin while_loop2
cargo fmt --verbose --package control_statements
*/

fn main() {
    let array = [10, 20, 30, 40, 50, 60];
    let mut i = 0;
    while i < 6 {
        print!("{}", array[i]);
        print!(" ");
        i = i + 1;
    }
}
