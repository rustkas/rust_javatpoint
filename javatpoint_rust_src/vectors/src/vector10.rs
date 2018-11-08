/*
cargo run -p vectors --bin vector10
cargo fmt --verbose --package vectors
*/

#[derive(Debug)]
enum Values {
    A(i32),
    B(f64),
    C(String),
}

fn main() {
    let v = vec![
        Values::A(5),
        Values::B(10.7),
        Values::C(String::from("javaTpoint")),
    ];
    for i in v {
        println!("{:?}", i);
    }
}
