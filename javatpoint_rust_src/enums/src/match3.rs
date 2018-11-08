/*
cargo run -p enums --bin match3
cargo fmt --verbose --package enums
*/

fn main() {
    Some(5);
}
fn Value(n: Option<i32>) {
    match n {
        Some(n) => println!("{}is a Number", n),
    }
}
