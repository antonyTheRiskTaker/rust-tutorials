// use rust_tutorials::{Draw, Button, Screen};

// struct SelectBox {
//     width: u32,
//     height: u32,
//     options: Vec<String>,
// }

// impl Draw for SelectBox {
//     fn draw(&self) {
//         // code to actually draw a select box
//     }
// }

use rust_tutorials::Post;

fn main() {
    // let screen = Screen {
    //     components: vec![
    //         Box::new(SelectBox {
    //             width: 75,
    //             height: 10,
    //             options: vec![
    //                 String::from("Yes"),
    //                 String::from("Maybe"),
    //                 String::from("No"),
    //             ],
    //         }),
    //         Box::new(Button {
    //             width: 50,
    //             height: 10,
    //             label: String::from("Ok"),
    //         }),
    //     ],
    // };

    // let screen = Screen {
    //     components: vec![Box::new(String::from("Hi"))],
    // };

    // screen.run();

    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    // assert_eq!("", post.content());

    let post = post.request_review();
    // assert_eq!("", post.content());

    let post = post.approve();
    
    assert_eq!("I ate a salad for lunch today", post.content());

}