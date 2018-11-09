/*
cargo run -p generics --bin lifetime01
cargo fmt --verbose --package generics
*/

fn main() {
    let a;
    {
        let b = 10;
        a = &b;
    }

    println!("a : {}", a);
}
