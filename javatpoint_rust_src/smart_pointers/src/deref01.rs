/*
cargo run -p smart_pointers --bin deref01
cargo fmt --verbose --package smart_pointers
*/

fn main() {
    let a = 20;
    let b = &a;
    if a == *b {
        println!("a and *b are equal");
    } else {
        println!("they are not equal");
    }
}
