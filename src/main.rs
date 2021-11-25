#[derive(Debug)]
struct A {
    a: i32,
}

#[derive(Debug)]
struct B {
    b: i32,
}

fn main() {
    let first = A { a: 42 };
    println!("{:?}", first);
}
