// fn test() {
//     println!("Test has been called...");
// }

// The "->" construct is Rust's return operator/keyword.
fn add_numbers(x: i32, y: i32) -> i32 {
    // println!("The sum is: {}", x + y);
    // (Line below) simply write an expression you'd like to return from a function. Pls drop the semicolon.
    // x + y
    // 10
    let result = x + y;
    // return result; // You can also return values like this.
    result
}

fn main() {
    println!("Hello, world!"); // A macro is an expression.
    // add_numbers(20, 30); // A function call is an expression.

    // let number = { // This is an expression because it returns a value.
    //     let x = 3;
    //     x + 1
    // };
    // println!("{}", number);
    let result = add_numbers(2, 3);
    println!("{}", result);
}