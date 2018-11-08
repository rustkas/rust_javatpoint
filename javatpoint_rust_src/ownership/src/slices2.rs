/*
cargo run -p ownership --bin slices2
cargo fmt --verbose --package ownership
*/
fn main() {
    {
        let str = String::from("hello world");
        let hello1 = &str[0..5];
        let hello2 = &str[..5];
        println!("{} {}", hello1, hello2);
    }

    {
        let str = String::from("hello world");
        let hello = &str[6..str.len()];
        let world = &str[6..];
        println!("{} {}", hello, world);
    }
}
