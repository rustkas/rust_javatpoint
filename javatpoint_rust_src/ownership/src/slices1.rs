/*
cargo run -p ownership --bin slices1
cargo fmt --verbose --package ownership
*/
fn main() {
    {
        let str = String::from("javaTpoint tutorial");
        let java_t_point = &str[0..10];
        let tutorial = &str[11..18];
        println!("{}; {}", java_t_point, tutorial);
    }

    {
        let str = String::from("javaTpoint tutorial");
        let java_t_point = &str[0..=9];
        let tutorial = &str[11..=18];
        println!("{}; {}", java_t_point, tutorial);
    }
}
