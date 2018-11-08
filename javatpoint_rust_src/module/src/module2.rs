/*
cargo run -p module --bin module2
cargo fmt --verbose --package module
*/
mod c {
    pub fn c() {
        println!("C is a structured programming language");
    }
}
mod cplus {
    pub fn cplus() {
        println!("C++ is an object-oriented programming language");
    }
}
fn main() {
    c::c();
    cplus::cplus();
}
