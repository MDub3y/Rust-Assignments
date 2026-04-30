/*
  Problem 41: RefCell<T> — Interior Mutability

  Create a struct Counter that wraps a RefCell<i32>.
  Implement methods increment(&self) and get(&self) -> i32 that mutate
  and read the counter through shared references.
  This demonstrates interior mutability.

  Run the tests for this problem with:
    cargo test --test refcell_counter_test
*/

use std::cell::RefCell;

pub struct Counter {
    pub value: RefCell<i32>,
}

impl Counter {
    pub fn new(initial: i32) -> Self {
        Self {
            value: RefCell::new(initial)
        }
    }

    pub fn increment(&self) {
        let mut val = self.value.borrow_mut();
        *val += 1;
    }

    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }
}
