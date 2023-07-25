use rust_book::chapters::c11_writing_automated_tests;

mod common;

#[test]
fn test_it_adds_two() {
    common::setup();
    assert_eq!(4, c11_writing_automated_tests::add_two(2));
}
