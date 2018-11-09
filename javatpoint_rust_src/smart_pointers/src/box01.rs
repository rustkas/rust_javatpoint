/*
cargo run -p smart_pointers --bin box01
cargo fmt --verbose --package smart_pointers
*/

fn main() {
    let a = Box::new(1);
    print!("value of a is : {}", a);
}
