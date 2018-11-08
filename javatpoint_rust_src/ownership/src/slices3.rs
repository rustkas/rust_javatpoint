/*
cargo run -p ownership --bin slices3
cargo fmt --verbose --package ownership
*/
fn main() {
    {
        let str = String::from("javaTpoint tutorial");
        let javatpoint = &str[..=9];
        println!("first word of the given string is {}", javatpoint);
    }
}
