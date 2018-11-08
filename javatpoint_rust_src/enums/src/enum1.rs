/*
cargo run -p enums --bin enum1
cargo fmt --verbose --package enums
*/

#[derive(Debug)]
enum ComputerLanguage {
    C,
    CPlusPlus,
    JAVA,
}
fn language_type(language_name: ComputerLanguage) {
    println!("{:?}", language_name);
}

fn main() {
    let c = ComputerLanguage::C;
    let cplus = ComputerLanguage::CPlusPlus;
    let java = ComputerLanguage::JAVA;

    language_type(c);
    language_type(cplus);
    language_type(java);

    language_type(ComputerLanguage::C);
    language_type(ComputerLanguage::CPlusPlus);
    language_type(ComputerLanguage::JAVA);
}
