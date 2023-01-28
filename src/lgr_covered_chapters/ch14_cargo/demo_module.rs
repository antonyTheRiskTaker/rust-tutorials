//! # Rust Tutorials
//! 
//! `rust_tutorials` is a collection of utilities to make performing certain
//! calculations more convenient.

/// Adds one to the number given.
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = rust_tutorials::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```
pub fn add_on(x: i32) -> i32 {
    x + 1
}