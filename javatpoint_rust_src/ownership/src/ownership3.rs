/*
cargo run -p ownership --bin ownership3
cargo fmt --verbose --package ownership
*/
fn main() {
    let s = String::from("javaTpoint");
    take_ownership(s);
    let ch = 'a';
    moves_copy(ch);
    println!("{}", ch);
}

fn take_ownership(str: String) {
    println!("{}", str);
}

fn moves_copy(c: char) {
    println!("{}", c);
}
