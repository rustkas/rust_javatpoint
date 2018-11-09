/*
cargo run -p generics --bin lifetime_ellision01
cargo fmt --verbose --package generics
*/

// Each parameter passed by the reference has got a distinct lifetime annotation.

#[allow(dead_code)]
fn fun1(_x: &i32, _y: &i32) {}

#[allow(dead_code)]
fn fun2<'a, 'b>(_x: &'a i32, _y: &'b i32) {}

fn main() {}
