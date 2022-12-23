fn main() {
    // let mut s = String::from("hello");

    // s.push_str(", world!"); // push_str() appends a literal to a String

    // println!("{}", s); // This will print `hello, world!`

    // let s1 = String::from("hello");
    // let s2 = s1;

    // println!("{}, world!", s1); // s1 is invalidated, so it won't compile

    // let s1 = String::from("hello");
    // let s2 = s1.clone();

    // println!("s1 = {}, s2 = {}", s1, s2);

    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}