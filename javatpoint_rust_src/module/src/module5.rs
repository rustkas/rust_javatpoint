/*
cargo run -p module --bin module5
cargo fmt --verbose --package module
*/
mod outer {
    pub fn a() {
        println!("function a");
    }
    fn b() {
        println!("function b");
    }

    mod inner {
        pub fn c() {
            println!("function c");
        }
        fn d() {
            println!("function d");
        }
    }
}
fn main() {
    outer::a();
    //outer::b();
    //outer::inner::c();
    //outer::inner::d();
}
