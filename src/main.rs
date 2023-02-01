use std::ops::Deref;

// Error! Without using Box<T> for this recursive type
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// use crate::List::{Cons, Nil};

// Implement the Deref trait for user-defined MyBox<T>
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Defining our own smart pointer
struct MyBox<T>(T);

impl <T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Cons(2, Cons(3, Nil))); // Error
    // let list = 
        // Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    // Using reference
    // let x = 5;
    // let y = &x;

    // Using a Box<T> that behaves like a reference
    // let x = 5;
    // let y= Box::new(x);

    // Using user-defined MyBox<T>
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    
}