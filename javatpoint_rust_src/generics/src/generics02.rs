/*
cargo run -p generics --bin generics02
cargo fmt --verbose --package generics
*/
use std::ops::Add;

fn main() {
    let a = vec![1, 2, 3, 4, 5];
    let b = vec![2.3, 3.3, 4.3, 5.3];
    let result = add(&a);
    let result1 = add(&b);
    println!("The value of result is {}", result);
    println!("The value of result1 is {}", result1);
}

fn add<T>(list: &[T]) -> T
where
    T: Add<Output = T>,
    T: Clone,
    T: Copy,
{
    let mut c: T = (*list.get(0).unwrap()).clone();
    for &item in list.iter() {
        c = c + item;
    }
    c
}
