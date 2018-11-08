/*
cargo run -p ownership --bin slices6
cargo fmt --verbose --package ownership
*/
fn main() {
    let arr = [100, 200, 300, 400, 500, 600];
    let mut i = 0;
    let a = &arr[1..=3];
    let len = a.len();
    println!("Elements of 'a' array:");
    while i < len {
        print!("{} ", a[i]);
        i = i + 1;
    }
    println!();
    for i in a[0..len].iter(){
        print!("{} ", i);
    }
}
