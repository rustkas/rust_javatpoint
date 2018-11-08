/*
cargo run -p control_statements --bin loop2
cargo fmt --verbose --package control_statements
*/

fn main() {
    let mut i = 1;
    loop {
        println!("Hello javaTpoint");
        if i == 7 {
            break;
        }
        i += 1;
    }
}
