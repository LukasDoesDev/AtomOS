#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn trivial_assertion_2() {
    assert_ne!("42", "answer to life, the universe and everything");
}
