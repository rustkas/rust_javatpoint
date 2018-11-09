/*
cargo run -p error --bin error1
cargo fmt --verbose --package error
*/
fn main() {
    let result = "100".parse::<u32>().ok().unwrap();
    println!("{:?}", result);

    let result: u32 = "100".parse().unwrap();
    println!("{:?}", result);

    //    let result:u32 ="A".parse().unwrap();
    //    println!("{:?}", result);
}
