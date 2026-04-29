/*
  Problem 39: Box<T> — Recursive List

  Define a recursive List enum using Box<T> with variants Cons(i32, Box<List>) and Nil.
  Implement a method sum() that returns the sum of all elements,
  and a method len() that returns the number of elements.

  Run the tests for this problem with:
    cargo test --test box_list_test
*/

#[derive(Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    pub fn sum(&self) -> i32 {
        let mut current = self;
        let mut total = 0;

        while let List::Cons(val, next) = current {
            total += val;

            current = next;
        }

        total
    }

    pub fn len(&self) -> usize {
        let mut current = self;
        let mut count = 0;

        while let List::Cons(_, next) = current {
            count += 1;

            current = next;
        }
        count
    }
}
