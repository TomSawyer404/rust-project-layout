mod something {
    #[derive(Debug)]
    pub struct A {
        pub a: i32,
    }

    #[derive(Debug)]
    pub struct B {
        pub b: i32,
    }
}

use crate::something::*;

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
