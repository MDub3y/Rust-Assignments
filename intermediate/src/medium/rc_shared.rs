/*
  Problem 40: Rc<T> — Shared Ownership

  Create a function that takes a String value, wraps it in Rc<String>,
  clones the Rc twice, and returns a tuple of the strong reference count
  and the string value itself (cloned for the return).
  This exercises shared ownership without copying the underlying data.

  Run the tests for this problem with:
    cargo test --test rc_shared_test
*/

use std::rc::Rc;

pub fn shared_ownership(value: String) -> (usize, String) {
    let rc_value = Rc::new(value);

    let _clone1 = Rc::clone(&rc_value);
    let _clone2 = Rc::clone(&rc_value);

    let count = Rc::strong_count(&rc_value);

    (count, (*rc_value).clone())
}
