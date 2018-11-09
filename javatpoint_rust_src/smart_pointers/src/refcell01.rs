/*
cargo run -p smart_pointers --bin refcell01
cargo fmt --verbose --package smart_pointers
*/
fn main() {
    let a = 15;
    let b = &mut a;
}
