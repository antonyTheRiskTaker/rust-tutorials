// use std::ops::Deref;

// Error! Without using Box<T> for this recursive type
// enum List {
//     Cons(i32, List),
//     Nil,
// }

// List using Box<T>
// enum List {
//     Cons(i32, Box<List>),
//     Nil,
// }

// List using Rc<T>
// enum List {
//     Cons(i32, Rc<List>),
//     Nil,
// }

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

// Implement the Deref trait for user-defined MyBox<T>
// impl<T> Deref for MyBox<T> {
//     type Target = T;

//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }

// Defining our own smart pointer
// struct MyBox<T>(T);

// impl <T> MyBox<T> {
//     fn new(x: T) -> MyBox<T> {
//         MyBox(x)
//     }
// }

// Running Code on Cleanup with the Drop Trait
// struct CustomSmartPointer {
//     data: String,
// }

// impl Drop for CustomSmartPointer {
//     fn drop(&mut self) {
//         println!("Dropping CustomSmartPointer with data `{}`!", self.data);
//     }
// }

// A library to keep track of how close a value is to a maximum value and warn
// when the value is at certain levels.

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
    // let x = 5;
    // let y = MyBox::new(x);

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    // let m = MyBox::new(String::from("Rust"));
    // hello(&m);

    // Running Code on Cleanup with the Drop Trait
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // };

    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };

    // println!("CustomSmartPointers created.");
    // drop(c);
    // println!("CustomSmartPointer dropped before the end of main.");

    // Using Rc<T> to Share Data

    // We are not allowed to have two lists using Box<T> that try to share
    // ownership of a third list
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // let c = Cons(4, Box::new(a));

    // let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    // println!("count after creating a = {}", Rc::strong_count(&a));
    // let b = Cons(3, Rc::clone(&a));
    // println!("count after creating b = {}", Rc::strong_count(&a));
    // {
    //     let c = Cons(4, Rc::clone(&a));
    //     println!("count after creating c = {}", Rc::strong_count(&a));
    // }
    // println!("count after c goes out of scope = {}", Rc::strong_count(&a));
    
    // Error: A Mutable Borrow to an Immutable Value
    // let x = 5;
    // let y = &mut x;

    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// fn hello(name: &str) {
//     println!("Hello, {name}!");
// }