/*
cargo run -p vectors --bin vector8
cargo fmt --verbose --package vectors
*/

fn main() {
    let mut v = Vec::new();
    v.push('j');
    v.push('a');
    v.push('v');
    v.push('a');
    for i in v {
        print!("{}", i);
    }
}
