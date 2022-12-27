// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}

//         fn seat_at_table() {}
//     }

//     mod serving {
//         fn take_order() {}

//         fn serve_order() {}

//         fn take_payment() {}
//     }
// }

// pub fn eat_at_restaurant() {
//     // Absolute path
//     crate::front_of_house::hosting::add_to_waitlist();

//     // Relative path
//     front_of_house::hosting::add_to_waitlist();
// }

// fn serve_order() {}

// mod back_of_house {
//     fn fix_incorrect_order() {
//         cook_order();
//         super::serve_order();
//     }

//     fn cook_order() {}
// }

// mod back_of_house {
//     pub struct Breakfast {
//         pub toast: String,
//         seasonal_fruit: String,
//     }

//     impl Breakfast {
//         pub fn summer(toast: &str) -> Breakfast {
//             Breakfast {
//                 toast: String::from(toast),
//                 seasonal_fruit: String::from("peaches"),
//             }
//         }
//     }
// }

// pub fn eat_at_restaurant() {
//     let mut meal = 
//         back_of_house::Breakfast::summer("Rye");
    
//     let meal2 = back_of_house::Breakfast {
//         toast: String::from("wheat"),
//         seasonal_fruit: String::from("peaches")
//     };

//     meal.toast = String::from("Wheat");
// }

// mod back_of_house {
//     // Once an enum is declared public, all its variants also become public.
//     pub enum Appetizer {
//         Soup,
//         Salad,
//     }
// }
// pub fn eat_at_restaurant() {
//     let order1 = back_of_house::Appetizer::Soup;
//     let order2 = back_of_house::Appetizer::Salad;
// }

// mod front_of_house {
//     pub mod hosting {
//         pub fn add_to_waitlist() {}
//     }
// }

// // use crate::front_of_house::hosting;
// // an idiomatic way to bring functions into scope
// use self::front_of_house::hosting;

// pub fn eat_at_restaurant() {
//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     // front_of_house::hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
//     hosting::add_to_waitlist();
// }

// use std::fmt;
// use std::io;

// fn function1() -> fmt::Result {
//     // --snip--
// }

// fn function2() -> io::Result<()> {
//     // --snip--
// }

// use std::fmt::Result;
// use std::io::Result as IoResult;

// fn function1() -> Result {
//     // --snip--
//     Ok(())
// }

// fn function2() -> IoResult<()> {
//     // --snip--
//     Ok(())
// }

// use rand::Rng;
// use rand::ErrorKind::Transient;
// use rand::CryptoRng;
use rand::{Rng, CryptoRng, ErrorKind::Transient};

// use std::io;
// use std::io::Write;
// use std::io::{self, Write};

// All public items underneath `io` are in scope.
use std::io::*;

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}