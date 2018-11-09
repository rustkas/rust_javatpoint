/*
cargo run -p generics --bin trait06
cargo fmt --verbose --package generics
*/

use std::fmt::{Debug, Display};
fn compare_prints<T: Debug + Display>(t: &T) {
    println!("Debug: '{:?}'", t);
    println!("Display: '{}'", t);
    assert_ne!(format!("{}", t), format!("{:?}", t))
}

#[allow(dead_code)]
fn fun1<T: Display + Debug, V: Clone + Debug>(_t: T, _v: V) -> i32 {
    0
}

#[allow(dead_code)]
fn fun2<T, V>(_t: T, _v: V) -> i32
where
    T: Display + Debug,
    V: Clone + Debug,
{
    0
}

fn main() {
    let string = "javaTpoint";
    compare_prints(&string);
}
