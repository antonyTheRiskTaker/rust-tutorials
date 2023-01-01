// use std::vec;
// use unicode_segmentation::UnicodeSegmentation;
// use std::collections::HashMap;

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }

fn main() {
    // let a = [1, 2, 3];
    // let mut v:Vec<i32> = Vec::new();
    // v.push(1);
    // v.push(2);
    // v.push(3);

    // {
    //     // The vector will be dropped once out of scope.
    //     let v2 = vec![1, 2, 3];
    // }

    // let mut v = vec![1, 2, 3, 4, 5];

    // let third = &v[2];
    // v.push(6);
    // println!("The third element is {}", third);

    // match v.get(20) {
    //     Some(third) => println!("The third element is {}", third),
    //     None => println!("There is no third element."),
    // }

    // let mut v = vec![1, 2, 3, 4, 5];

    // Iterate vector elements
    // for i in &mut v {
    //     *i += 50;
    //     println!("{}", i);
    // }
    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];

    // match &row[1] {
    //     SpreadsheetCell::Int(i) => println!("{}", i),
    //     _ => println!("Not an integer!"),
    // };

    // Strings are stored as a collection of UTF-8 encoded bytes
    // let s1 = String::new();
    // let s2 = "initial contents";
    // let s3 = s2.to_string();
    // let s4 = String::from("initial contents");

    // One way to append a string
    // let mut s = String::from("foo");
    // s.push_str("bar");
    // s.push('!'); // `s` becomes "foobar!"

    // Another way to append a string
    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");

    // Moving ownership of `s1` to `s3` and taking all the characters in `s2` and append them to `s3`
    // let s3 = s1 + &s2;

    // let s3 = format!("{}{}", s1, s2);

    // println!("{}", s1);

    // let hello = String::from("नमस्ते");

    // for b in "नमस्ते".bytes() {
    //     println!("{}", b);
    // }

    // for c in "नमस्ते".chars() {
    //     println!("{}", c);
    // }

    // for g in "नमस्ते".graphemes(true) {
    //     println!("{}", g);
    // }

    //* HashMap section

    // let mut scores = HashMap::new();

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Yellow"), 50);

    // let team_name = String::from("blue");
    // let score = scores.get(&team_name).copied().unwrap_or(0);
    // println!("score: {score}");

    // for (key, value) in &scores {
    //     println!("{key}: {value}");
    // }

    // let field_name = String::from("Favorite color");
    // let field_value = String::from("Blue");

    // let mut map = HashMap::new();
    // map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!

    // scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 25);
    
    // println!("{:?}", scores);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    // println!("{:#?}", scores);

    // let text = "hello world wonderful world";

    // let mut map = HashMap::new();

    // for word in text.split_whitespace() {
    //     let count = map.entry(word).or_insert(0);
    //     *count += 1;
    // }

    // println!("{:#?}", map);

    // let mut vec = vec![1, 5, 10, 2, 15];

    // vec.sort();

    // println!("{:#?}", vec);

}
