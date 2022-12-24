// struct User {
//     username: String,
//     email: String,
//     sign_in_count: u64,
//     active: bool,
// }

// struct Color(i32, i32, i32);
// struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    // let mut user1 = User {
    //     email: String::from("antony.lo1453@gmail.com"),
    //     username: String::from("antony1453"),
    //     active: true,
    //     sign_in_count: 1
    // };

    // let name = user1.username;
    // user1.username = String::from("wallace123");

    // let user2 = build_user(
    //     String::from("kyle@gmail.com"), 
    //     String::from("kyle123")
    // );

    // let user3 = User {
    //     email: String::from("james@gmail.com"),
    //     username: String::from("james123"),
    //     ..user2
    // };

    // let width1 = 30;
    // let height1 = 50;

    // let rect = (30, 50);
    let rect = Rectangle {
        width: 30,
        height: 50
    };

    println!("rect: {:#?}", rect);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
}

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }

// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn build_user(email: String, username: String) -> User {
//     User {
//         // email: email,
//         // username: username,
//         email, // can be simplified like this
//         username, // can be simplified like this
//         active: true,
//         sign_in_count: 1
//     }
// }