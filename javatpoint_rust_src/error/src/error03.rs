/*
cargo run -p error --bin error3
cargo fmt --verbose --package error
*/
enum Value {
    Val,
}

fn get_number(_: Value) -> i32 {
    5
}
fn find_number(val: Value) -> &'static str {
    match get_number(val) {
        7 => "seven",
        8 => "eight",
        _ => unreachable!(),
    }
}

fn main() {
    println!("{}", find_number(Value::Val));
}
