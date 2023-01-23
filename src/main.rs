// use std::thread;

// #[derive(Debug, PartialEq, Copy, Clone)]
// enum ShirtColor {
//     Red,
//     Blue,
// }

// struct Inventory {
//     shirts: Vec<ShirtColor>,
// }

// impl Inventory {
//     fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
//         user_preference.unwrap_or_else(|| self.most_stocked())
//     }

//     fn most_stocked(&self) -> ShirtColor {
//         let mut num_red = 0;
//         let mut num_blue = 0;

//         for color in &self.shirts {
//             match color {
//                 ShirtColor::Red => num_red += 1,
//                 ShirtColor::Blue => num_blue += 1,
//             }
//         }
//         if num_red > num_blue {
//             ShirtColor::Red
//         } else {
//             ShirtColor::Blue
//         }
//     }
// }

// #[derive(Debug)]
// struct Rectangle {
//     width: u32,
//     height: u32,
// }

fn main() {
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );

    // let example_closure = |x| x;

    // let s = example_closure(String::from("hello"));
    // let n = example_closure(5);

    // * Capturing References or Moving Ownership
    // let list = vec![1, 2, 3];
    // println!("Before defining closure: {:?}", list);

    // thread::spawn(move || println!("From thread: {:?}", list))
        // .join()
        // .unwrap();

    // let only_borrows = || println!("From closure: {:?}", list);
    // let mut borrows_mutably = || list.push(7);

    // println!("Before calling closure: {:?}", list);
    // only_borrows();
    // borrows_mutably();
    // println!("After calling closure: {:?}", list);

    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // This doesn't work (see explanation below Listing 13-8)
    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });

    // let mut num_sort_operations = 0;
    // list.sort_by_key(|r| {
    //     num_sort_operations += 1;
    //     r.width
    // });

    // closure that applies `FnOnce`
    // list.sort_by_key(|r| r.width);
    // println!("{:#?}", list);
    // println!("{:#?}, sorted in {num_sort_operations} operations", list);

    //* Listing 13-10: Creating an iterator
    // let v1 = vec![1, 2, 3];
    // let v1_iter = v1.iter();

    // for val in v1_iter { // Listing 13-11
    //     println!("Got: {val}");
    // }
}