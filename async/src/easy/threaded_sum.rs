/*
  Problem 71: Multithreaded Sum

  Write a function that takes a Vec<i32> and splits it into two halves.
  Sum each half in a separate thread using std::thread::spawn and return
  the total sum.

  Run the tests for this problem with:
    cargo test --test threaded_sum_test
*/

use std::thread;

pub fn threaded_sum(v: Vec<i32>) -> i32 {

    let mid = v.len() / 2;
    let mut v2 = v;
    let v1 = v2.split_off(mid);

    let handle1 = thread::spawn( move || {
      v1.iter().fold(0i32, |acc, &x| acc + x)
    });

    let handle2 = thread::spawn(move || {
      v2.iter().fold(0i32, |acc, &x| acc + x)
    });

    let res1 = handle1.join().unwrap();
    let res2 = handle2.join().unwrap();

    res1 + res2
}
