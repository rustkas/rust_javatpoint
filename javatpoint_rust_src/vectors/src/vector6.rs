/*
cargo run -p vectors --bin vector6
cargo fmt --verbose --package vectors
*/

fn main() {
    let v = vec![20, 30, 40, 50];
    print!("Elements of vector are : ");
    for i in v {
        print!("{} ", i);
    }
}
