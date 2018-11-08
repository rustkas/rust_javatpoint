/*
cargo run -p ownership --bin slices4
cargo fmt --verbose --package ownership
*/
fn main() {
    let str = String::from("Computer Science");
    let _first_word = first_word(&str[..]); //first_word function finds the first word of the string.
    let s = "Computer Science"; //string literal
    let _first_word = first_word(&s[..]); // first_word function finds the first word of the string.
    let _first_word = first_word(s); //string slice is same as string literal. Therefore, it can also be
                                     // written in this way also.
}

fn first_word(slice: &str) -> String {
    let mut string: String = slice.to_string();
    string = string.trim().to_string();
    if string.len() == 0 {
        return slice.to_string();
    }

    let vector: Vec<_> = string.split(' ').collect();
    // println!("{}", vector[0]);
    if vector.len() > 0 {
        return vector[0].to_string();
    }
    slice.to_string()
}
