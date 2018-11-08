/*
cargo run -p vectors --bin vector7
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut v = vec![20, 30, 40, 50];
    println!("The mutable vector is {:?}", v);
    print!("New elements of vector are : ");
    for i in &mut v {
        *i += 20;
        print!("{} ", i);
    }
}
