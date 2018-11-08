/*
cargo run -p vectors --bin vector1
cargo fmt --verbose --package vectors
*/

fn main() {
    let v: Vec<i32> = Vec::new();
    println!("{:?}", v);

    let v = vec![10, 20, 30, 40, 50];
    println!("{:?}", v);

    println!("--------------------------------");

    for i in 0..=10 {
        let v = vec![2; i];
        println!("{:?}", v);
    }
}
