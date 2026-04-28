/*
  Problem 91: Async Shared State — Mutex Vec

  Rewrite Problem 74 / 79 using tokio::sync::Mutex to share a Vec<i32>.
  Spawn 5 tasks, each pushing 10 numbers into the vector.
  Return the length of the final vector.

  Run the tests for this problem with:
    cargo test --test async_shared_vec_test
*/

use tokio::sync::Mutex;
use std::sync::Arc;

pub async fn async_shared_vec() -> usize {
    let shared_vec = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    for _ in 0..5 {
      let vec_clone = Arc::clone(&shared_vec);

      let handle = tokio::spawn(async move {
        for i in 0..10 {
          let mut v = vec_clone.lock().await;
          v.push(i);
        }
      });
      handles.push(handle);
    }

    for handle in handles {
      let _ = handle.await;
    }

    let final_vec = shared_vec.lock().await;

    final_vec.len()
}
