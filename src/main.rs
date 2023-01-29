// Importing code according to the internal structure of the library
// that is more relevant to those who develop it.
// use rust_tutorials::kinds::PrimaryColor;
// use rust_tutorials::utils::mix;

// Using the re-exported types and functions
use rust_tutorials::mix;
use rust_tutorials::PrimaryColor;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}