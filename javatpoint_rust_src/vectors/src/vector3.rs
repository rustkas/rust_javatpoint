/*
cargo run -p vectors --bin vector3
cargo fmt --verbose --package vectors
*/

fn value(n: Option<&i32>) {
    match n {
        Some(n) => println!("Fourth element of a vector is {}", n),
        None => println!("None"),
    }
}
fn main() {
    let v = vec![20, 30, 40, 50];
    let a: Option<&i32> = v.get(3);
    value(a);

    for i in 0..v.len() {
        value(v.get(i));
    }
    value(Option::None);
}
