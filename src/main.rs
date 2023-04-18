// use std::slice;

// extern "C" {
//     fn abs(input: i32) -> i32;
// }

// static HELLO_WORLD: &str = "Hello, world!";

// static mut COUNTER: u32 = 0;

// fn add_to_count(inc: u32) {
//     unsafe {
//         COUNTER += inc;
//     }
// }

// trait Pilot {
//     fn fly(&self);
// }

// trait Wizard {
//     fn fly(&self);
// }

// struct Human;

// impl Pilot for Human {
//     fn fly(&self) {
//         println!("This is your captain speaking.");
//     }
// }

// impl Wizard for Human {
//     fn fly(&self) {
//         println!("Up!");
//     }
// }

// impl Human {
//     fn fly(&self) {
//         println!("*waving arms furiously*");
//     }
// }

// trait Animal {
//     fn baby_name() -> String;
// }

// struct Dog;

// impl Dog {
//     fn baby_name() -> String {
//         String::from("Spot")
//     }
// }

// impl Animal for Dog {
//     fn baby_name() -> String {
//         String::from("puppy")
//     }
// }

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

// TODO: continue from Listing 19-22

fn main() {
    // let mut num = 5;

    // let r1 = &num as *const i32;
    // let r2 = &mut num as *mut i32;

    // Creating a raw pointer to an arbitrary memory address
    // let address = 0x012345usize;
    // let r = address as *const i32;

    // Dereferencing raw pointers within an `unsafe` block
    // unsafe {
    //     println!("r1 is: {}", *r1);
    //     println!("r2 is: {}", *r2);
    // }

    // unsafe {
    //     dangerous();
    // }

    // let mut v = vec![1, 2, 3, 4, 5, 6];

    // let r = &mut v[..];

    // let (a, b) = r.split_at_mut(3);

    // assert_eq!(a, &mut [1, 2, 3]);
    // assert_eq!(b, &mut [4, 5, 6]);

    // let address = 0x01234usize;
    // let r = address as *mut i32;

    // let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    // unsafe {
        // println!("Absolute value of -3 according to C: {}", abs(-3));
    // }

    // println!("name is: {HELLO_WORLD}");

    // add_to_count(3);

    // unsafe {
    //     println!("COUNTER: {COUNTER}");
    // }

    // let person = Human;
    // Pilot::fly(&person);
    // Wizard::fly(&person);
    // person.fly();

    // println!("A baby dog is called a {}", <Dog as Animal>::baby_name());
    
}

// unsafe fn dangerous() {}

// fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     let len = values.len();
//     let ptr = values.as_mut_ptr();

//     assert!(mid <= len);

//     unsafe {
//         (
//             slice::from_raw_parts_mut(ptr, mid),
//             slice::from_raw_parts_mut(ptr.add(mid), len - mid),
//         )
//     }

    // (&mut values[..mid], &mut values[mid..])
// }