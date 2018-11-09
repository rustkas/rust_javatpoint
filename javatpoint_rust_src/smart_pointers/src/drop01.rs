/*
cargo run -p smart_pointers --bin drop01
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
struct Example {
    a: i32,
}
impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the instance of Example with data : {}", self.a);
    }
}
fn main() {
    println!("Instances of Example type are created");
    let a1 = Example { a: 10 };
    let b1 = Example { a: 20 };
    println!("sum of values are {}", a1.a + b1.a);
}
