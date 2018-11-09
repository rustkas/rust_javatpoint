/*
cargo run -p generics --bin generics01
cargo fmt --verbose --package generics
*/
fn main() {
    let _x: Option<i32> = Some(10); // 'T' is of type i32.
    let _x: Option<bool> = Some(true); // 'T' is of type bool.
    let _x: Option<f64> = Some(10.5); // 'T' is of type f64.
    let _x: Option<char> = Some('b'); // 'T' is of type char.
                                      // let x : Option<i32> = Some(10.8);
}
