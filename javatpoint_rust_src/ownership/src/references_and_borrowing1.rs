/*
cargo run -p ownership --bin references_and_borrowing1
cargo fmt --verbose --package ownership
*/
fn main() {
    let str = String::from("javaTpoint");
    let len = calculate_length(&str);
    println!("length of the string '{}' is {}", &str, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
