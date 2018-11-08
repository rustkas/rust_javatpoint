/*
cargo run -p control_statements --bin for_loop3
cargo fmt --verbose --package control_statements
*/

fn main() {
    let fruits = ["mango", "apple", "banana", "litchi", "watermelon"];
    for a in fruits.iter() {
        print!("{} ", a);
    }
}
