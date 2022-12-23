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
    
    // let mut s = String::from("hello world");

    // let word = first_word(&s); // word will get the value 5

    // s.clear(); // this empties the String, making it equal to ""

    // word still has the value 5 here, but there's no more string that we
    // could meaningfully use the value 5 with. word is now totally invalid!
    
    // let mut s = String::from("hello world");

    // let word = first_word(&s);

    // s.clear(); // error!

    // println!("the first word is: {}", word);

    // let my_string = String::from("hello world");

    // `first_word` works on slices of `String`s, whether partial or whole
    // let word = first_word(&my_string[0..6]);
    // let word = first_word(&my_string[..]);
    // `first_word` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    // let word = first_word(&my_string);
    
    // let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or
    // whole
    // let word = first_word(&my_string_literal[0..6]);
    // let word = first_word(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    // let word = first_word(my_string_literal);
    // println!("{}", word);

    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

// fn change(some_string: &mut String) {
//     some_string.push_str(", world");
// }

// fn dangle() -> &String { // dangle returns a reference to a String
    // let s = String::from("hello"); // s is a new String

    // &s // we return a reference to the String, s
// } // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!

// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return i;
//         }
//     }

//     s.len()
// }

// fn first_word(s: &String) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }

// fn first_word(s: &str) -> &str {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate() {
//         if item == b' ' {
//             return &s[0..i];
//         }
//     }

//     &s[..]
// }