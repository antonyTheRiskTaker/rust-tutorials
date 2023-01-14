use rust_tutorials;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, rust_tutorials::add_two(2));
}