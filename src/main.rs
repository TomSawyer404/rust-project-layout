mod something;

use crate::something::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
