/*
cargo run -p smart_pointers --bin drop03
cargo fmt --verbose --package smart_pointers
*/

#[derive(Debug)]
struct Example {
    a: String,
}

impl Drop for Example {
    fn drop(&mut self) {
        println!("Dropping the instance of Example with data : {}", self.a);
    }
}

fn main() {
    let a1 = Example {
        a: String::from("Hello"),
    };
    println!("{:?}", &a1);

    //    drop(a1);

    drop(&a1);
    let b1 = Example {
        a: String::from("World"),
    };
    println!("{:?}", &b1);
    println!("{:?}", &a1);
    println!("Instances of Example type are created");
}
