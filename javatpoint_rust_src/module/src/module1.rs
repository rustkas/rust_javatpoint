/*
cargo run -p module --bin module1
cargo fmt --verbose --package module
*/
mod a {
    pub fn single_module() {
        println!("Single module");
    }
}
fn main() {
    a::single_module();
}
