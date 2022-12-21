// use std::io;

fn main() {
    // let a = [1, 2, 3, 4, 5];
    
    // let index = read_index();

    // let element = a[index];

    // println!("The value of the element at index {index} is: {element}");
    
    // loop { // It turns into an infinite loop when run.
    //     println!("again!");
    // }

    // let mut count = 0;
    // 'counting_up: loop {
    //     println!("count = {count}");
    //     let mut remaining = 10;

    //     loop {
    //         println!("remaining = {remaining}");
    //         if remaining == 9 {
    //             break;
    //         }
    //         if count == 2 {
    //             break 'counting_up;
    //         }
    //         remaining -= 1;
    //     }

    //     count += 1;
    // }
    // println!("End count = {count}");

    // let mut number = 3;

    // while number != 0 {
    //     println!("{number}!");

    //     number -= 1;
    // }

    // println!("LIFTOFF!!!");
    
    // let a = [10, 20, 30, 40, 50];
    // let mut index = 0;

    // while index < 5 {
    //     println!("the value is: {}", a[index]);

    //     index += 1;
    // }

    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}

// fn read_index() -> usize {
//     println!("Please enter an array index.");

//     let mut index = String::new();
//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");
    
//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");
    
//     return index;
// }