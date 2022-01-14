use crate::List::{Cons, Nil};
use std::ops::Deref;
use std::rc::Rc;

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // i32 example
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    // hello example
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // multiple owners example
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating _b = {}", Rc::strong_count(&a));
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating _c = {}", Rc::strong_count(&a));
    }
    println!(
        "count after _c goes out of scope = {}",
        Rc::strong_count(&a)
    );
}
