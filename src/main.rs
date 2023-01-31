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

fn main() {
    // let b = Box::new(5);
    // println!("b = {b}");

    // let list = Cons(1, Cons(2, Cons(3, Nil))); // Error
    // let list = 
        // Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
    // TODO: continue from `Using Box<T> Like a Reference`
}