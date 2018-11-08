/*
cargo run -p module --bin module4
cargo fmt --verbose --package module
*/
mod A {
    fn a() {
        // block of statements.
    }
}
mod B {
    fn b() {
        // block of statements.
    }
    mod C {
        fn c() {
            // block of statements.
        }
    }
}
