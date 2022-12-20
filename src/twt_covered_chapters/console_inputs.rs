use std::io;

fn main() {
    println!("Hello, world!");
    let mut input = 4;
    
    // "&mut" means we're creating a mutable reference to the variable input. Apparently Rust is a pass-by-value language.
    io::stdin()
        .read_line(&mut input);
        // .expect("failed to read line");
    println!("{}", input);
}
