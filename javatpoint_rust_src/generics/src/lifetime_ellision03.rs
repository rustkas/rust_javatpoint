/*
cargo run -p generics --bin lifetime_ellision03
cargo fmt --verbose --package generics
*/

// If multiple parameters passed by reference and one of them is &self or &mut self,
// then the lifetime of self is assigned to all the elided output lifetimes.

#[allow(dead_code)]
trait A{
fn fun1(&self, _x : &str)->&str{""}
}

#[allow(dead_code)]
trait B{
    fn fun<'a,'b>(& 'a mut self, _x : & 'b str) -> & 'a str{""}
}


fn main() {}
