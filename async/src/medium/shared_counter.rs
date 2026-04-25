/*
  Problem 74: Shared State — Mutex Counter

  Write a function that spawns 10 threads. Each thread should increment a shared
  counter 100 times. Use Arc<Mutex<i32>> to share the counter safely.
  Wait for all threads to finish and return the final counter value.

  Run the tests for this problem with:
    cargo test --test shared_counter_test
*/

use std::sync::{Arc, Mutex};
use std::thread;

pub fn multithreaded_counter() -> i32 {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 1..=10 {
      let count = Arc::clone(&counter);
      let handle = thread::spawn(move ||{
        for _ in 1..=100 {
          let mut num = count.lock().unwrap();
          *num += 1;
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
    let res = *counter.lock().unwrap();

    res
}
