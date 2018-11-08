/*
cargo run -p module --bin use4
cargo fmt --verbose --package module
*/
mod a {
    fn x() -> u8 {
        5
    }

    pub mod example {
        use super::x;

        pub fn foo() {
            println!("{}", x());
        }
    }
}

fn main() {
    a::example::foo();
}
