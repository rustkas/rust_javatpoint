/*
cargo run -p enums --bin match2
cargo fmt --verbose --package enums
*/

fn main() {
    even_number(2);
    even_number(3);
}
fn even_number(n: i32) {
    let num = n;
    match checked_even(n) {
        None => println!("None"),

        Some(n) => {
            if n == 0 {
                println!("{} is a even number", num);
            } else {
                println!("{} is a odd number", num);
            }
        }
    }
}
fn checked_even(number: i32) -> Option<i32> {
    Some(number % 2)
}
