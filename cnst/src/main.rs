fn main() {
    println!("Hello, world!");
    let a = A::new();
}

struct A {
    b: Box<B>,
}

impl A {
    fn new() -> Self {
        Self {
            b: Box::new(B::new()),
        }
    }
}

struct B {
    a: Box<A>,
}

impl B {
    fn new() -> Self {
        Self {
            a: Box::new(A::new()),
        }
    }
}
