/*
cargo run -p ownership --bin ownership2
cargo fmt --verbose --package ownership
*/
fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = v1;
    println!("{:?}", v2);
    //println!("{:?}",v1);
}
