/*
cargo run -p module --bin module3
cargo fmt --verbose --package module
*/
mod a {
    pub fn a() {
        println!("a module");
    }
    pub mod b {
        pub fn b() {
            println!("b module");
        }
    }
}
fn main() {
    a::a();
    a::b::b();
}
