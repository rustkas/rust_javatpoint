/*
cargo run -p generics --bin lifetime_ellision02
cargo fmt --verbose --package generics
*/

// If the single parameter is passed by reference, then the lifetime of that parameter is assigned
// to all the elided output lifetimes.

#[allow(dead_code)]
fn fun1(_x : i32, _y : &i32) -> &i32 {&0}

#[allow(dead_code)]
fn fun<'a>(_x : i32, _y : & 'a i32) -> & 'a i32{&0}

fn main() {}
