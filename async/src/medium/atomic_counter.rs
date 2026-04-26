/*
  Problem 79: Atomic Counter

  Rewrite the multithreaded counter problem (Problem 74) using AtomicI32
  instead of Mutex<i32>. Compare the performance and complexity.
  Show use of fetch_add and Ordering.

  Run the tests for this problem with:
    cargo test --test atomic_counter_test
*/

use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;

pub fn atomic_counter() -> i32 {
  let counter = Arc::new(AtomicI32::new(0));
  let mut handles = vec![];

  for _ in 1..=10 {
    let count = Arc::clone(&counter);
    let handle = thread::spawn(move ||{
      for _ in 1..=100 {
        count.fetch_add(1, Ordering::SeqCst);
      }
    });

    handles.push(handle);
  }

  for handle in handles {
    handle.join().unwrap();
  }

  // mutex guard i sdropped here with the semicolon
  // this can't be the tail expression, as in that case,
  // the counter is cropped while the mutex guard is still alive!
  let res = counter.load(Ordering::SeqCst);

  res
}
