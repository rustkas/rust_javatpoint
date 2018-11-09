/*
cargo run -p generics --bin trait05
cargo fmt --verbose --package generics
*/

// Trait limitation

//The trait which we are implementing must be defined by us. For example:
//If we define the HasArea trait, then we can implement this trait for the type i32.
//However, we could not implement the toString trait defined by the Rust for the type i32
//as both the type and trait are not defined in our crate.

use std::string::ToString;

trait HasArea {
    fn area(&self) -> f64;
}
impl HasArea for i32 {
    fn area(&self) -> f64 {
        *self as f64
    }
}

struct Triangle {
    base: f64,
    height: f64,
}
impl ToString for Triangle {
    fn to_string(&self) -> String {
        format!("[base: {}; height: {}", self.base, self.height)
    }
}
impl HasArea for Triangle {
    fn area(&self) -> f64 {
        0.5 * (self.base * self.height)
    }
}

fn main() {
    let a = Triangle {
        base: 10.5,
        height: 17.4,
    };
    let i32_value: i32 = 9;
    let i32_area = i32_value.area();
    println!("value = {} has {} area", i32_value, i32_area);
    println!("{}", a.to_string());
}
