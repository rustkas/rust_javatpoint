/*
cargo run -p error --bin error4
cargo fmt --verbose --package error
*/
enum Value {
    FIVE = 5,
    SEVEN = 7,
    EIGHT = 8,
}

fn get_number(value: Value) -> i32 {
    value as i32
}

fn find_number(val: Value) -> &'static str {
    match get_number(val) {
        7 => "seven",
        8 => "eight",
        _ => unreachable!(),
    }
}

fn main() {
    println!("{}", find_number(Value::SEVEN));
    println!("{}", find_number(Value::EIGHT));
    println!("{}", find_number(Value::FIVE));
}
