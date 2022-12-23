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

    // let mut s = String::from("hello");

    // println!("{s}");
    // change(&mut s);
    // println!("{s}");

    // {
    //     let r1 = &mut s;
    // }

    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {} and {}", r1, r2, r3);

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // variables r1 and r2 will not be used after this point
    // println!("{} and {}", r1, r2);

    // let r3 = &mut s; // no problem
    // println!("{}", r3);

    // let reference_to_nothing = dangle();
    
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String { // dangle returns a reference to a String
    // let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
