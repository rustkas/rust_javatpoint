/*
cargo run -p vectors --bin string11
cargo fmt --verbose --package vectors
*/

fn main() {
    let s = "C is a programming language";
    for i in s.chars()
        {
            print!("{}",i);
        }
}
