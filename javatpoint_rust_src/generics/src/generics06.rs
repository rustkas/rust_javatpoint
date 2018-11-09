/*
cargo run -p generics --bin generics06
cargo fmt --verbose --package generics
*/

fn main() {
    {
        let mut v = Vec::new(); // creating a vector.
        v.push(10); // inserts integer value into the vector. Therefore, v is of i32 type.
        println!("{:?}", v); // prints the value of v.
    }
    {
        let mut v = Vec::new(); // creating a vector.
        v.push('a');
        println!("{:?}", v); // prints the value of v.
    }
    {
        let v: Vec<bool> = Vec::new();
        println!("{:?}", v);
    }
    {
        let v = Vec::<bool>::new();
        println!("{:?}", v);
    }
}
