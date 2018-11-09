/*
cargo run -p smart_pointers --bin drop01
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
struct Example {
    a: String,
}
impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the instance of Example with data : {}", self.a);
    }
}
fn main() {
    let mut a1 = Example {
        a: String::from("Hello"),
    };
    a1.drop();
    let b1 = Example {
        a: String::from("World"),
    };
    println!("Instances of Example type are created");
}
