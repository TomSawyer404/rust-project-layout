mod something;
use crate::something::a::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
