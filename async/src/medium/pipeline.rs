/*
  Problem 77: Message Passing — Data Pipeline

  Create a three-stage pipeline using mpsc channels:
  1. Producer: Sends numbers 1..=5.
  2. Processor: Receives numbers, squares them, and sends to next stage.
  3. Consumer: Receives squared numbers and sums them.
  Implement this using three threads and return the final sum.

  Run the tests for this problem with:
    cargo test --test pipeline_test
*/

use std::sync::mpsc;
use std::thread;

pub fn data_pipeline() -> i32 {
    let (tx, rx) = mpsc::channel();
    let (tx1, rx1) = mpsc::channel();

    let handle1 = thread::spawn(move || {
      for i in 1..=5 {
        tx.send(i).unwrap();
      }
    });

    let handle2 = thread::spawn(move || {
      for x in rx {
        tx1.send(x*x).unwrap();
      }
    });

    let handle3 = thread::spawn(move || {
      let mut local_sum = 0;
      for x in rx1 {
        local_sum += x;
      }
      local_sum
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let sum = handle3.join().unwrap();

    sum
}
