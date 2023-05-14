use std::rc::Rc;

fn main() {
    let list = Rc::new(Cons(1, Rc::new(Cons(2, Rc::new(Cons(3, Rc::new(Nil)))))));
    println!("{:#?}", list);

    let blist = Cons(10, Rc::clone(&list));
    let clist = Cons(20, Rc::clone(&list));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    let c = MySmartPointer {
        data: String::from("my stuff"),
    };
    let d = MySmartPointer {
        data: String::from("other stuff"),
    };

    println!("MySmartPointer:");
    drop(c);
}

use std::ops::Deref;

use List::{Cons, Nil};
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}", name);
}

struct MySmartPointer {
    data: String,
}

impl Drop for MySmartPointer {
    fn drop(&mut self) {
        println!("Dropping MySmartPointer with: {}", self.data);
    }
}
