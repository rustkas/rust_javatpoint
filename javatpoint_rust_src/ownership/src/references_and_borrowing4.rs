/*
cargo run -p ownership --bin references_and_borrowing4
cargo fmt --verbose --package ownership
*/
fn main() {
    {
        let mut str = String::from("javaTpoint");
        let _a = &mut str;
        // let _b= &mut str;
    }

    {
        let mut str = String::from("javaTpoint");
        //let _a= &str;
        //let _b=&str;
        let _c = &mut str;
    }
}
