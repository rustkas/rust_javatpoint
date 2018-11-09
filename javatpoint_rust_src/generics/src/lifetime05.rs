/*
cargo run -p generics --bin lifetime05
cargo fmt --verbose --package generics
*/

struct Example<'a> {
    x: &'a i32,
}
impl<'a> Example<'a> {
    fn display(&self) {
        print!("Value of x is : {}", self.x);
    }
}
#[allow(dead_code)]
fn fun1<'a>(_x: &'a i32, _y: &'a i32) -> &'a i32 {
    &0
}

#[allow(dead_code)]
fn fun2<'a, 'b>(_x: &'a i32, _y: &'b i32) -> (&'a i32, &'b i32) {
    (&0, &0)
}

#[allow(dead_code)]
fn fn3() -> &'static str {
    let s: &'static str = "javaTpoint tutorial";
    s
}

fn main() {
    let y = &90;
    let b = Example { x: y };
    b.display();
}
