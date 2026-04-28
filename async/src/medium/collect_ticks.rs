/*
  Problem 95: Async Streams — Collect Ticks

  Write an async function that creates a tokio::time::interval (1ms).
  Collect the first 5 "ticks" (Instant values) from the interval stream
  into a Vec and return it.

  Run the tests for this problem with:
    cargo test --test collect_ticks_test
*/

use tokio::time::{interval, Duration, Instant};

pub async fn collect_ticks() -> Vec<Instant> {
    let mut timer = interval(Duration::from_millis(1));
    let mut ticks = Vec::new();

    for _ in 0..5 {
      let now = timer.tick().await;

      ticks.push(now);
    }

    ticks
}
