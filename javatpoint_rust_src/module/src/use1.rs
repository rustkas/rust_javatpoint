/*
cargo run -p module --bin use1
cargo fmt --verbose --package module
*/
pub mod a {
    pub mod b {
        pub mod c {
            pub fn nested_modules() {
                println!("Nested Modules");
            }
        }
    }
}

use a::b::c::nested_modules;
fn main() {
    nested_modules();
}
