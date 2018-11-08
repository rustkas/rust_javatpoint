/*
cargo run -p module --bin module6
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

fn main() {
    a::b::c::nested_modules();
}
