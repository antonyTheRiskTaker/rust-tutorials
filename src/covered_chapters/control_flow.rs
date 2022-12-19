fn main() {
    // let cond = (2 as f32) <= 2.2;
    // let cond2 = false || !cond;
    // println!("{}", cond2);

    let food = "bread";

    if food == "cookie" {
        println!("I like cookies too!");
        // if {} // Can also insert an if statement inside an if statement.
    } else if food == "fruit" {
        println!("That sounds healthy!");
    } else if food == "bread" {
        println!("That sounds boring!");
    } else {
        println!("Oh, that's too bad!");
    }
}