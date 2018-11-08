/*
cargo run -p vectors --bin vector9
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut v1: Vec<i32>;

    {
        let v = vec![30, 40, 50];
        v1 = v;
        println!("{:?}", &v1);
    } // => v is freed here as it goes out of the scope.

    {
        let v = vec![300, 400, 500];
        v1 = v;
        println!("{:?}", &v1);
    } // => v is freed here as it goes out of the scope.

    v1.push('a' as i32);
    println!("{:?}", v1);
}
