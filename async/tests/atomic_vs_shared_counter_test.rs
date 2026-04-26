/* 
  run this test using: 
    cargo test --release --test atomic_vs_shared_counter_test -- --nocapture

  --release flag: builds and run ur code using release profile rather that 
      the default dev profile.
      used to test performance critical code (debug mode is unoptimized)

  --nocapture flag: shows every println!
*/


use std::time::Instant;
use r#async::medium::atomic_counter::atomic_counter;
use r#async::medium::shared_counter::multithreaded_counter;

#[test]
fn compare_speeds() {

    let start_m = Instant::now();
    let _ = multithreaded_counter();
    let duration_m = start_m.elapsed();
    println!("Mutex   Time: {:?}", duration_m);

    let start_a = Instant::now();
    let _ = atomic_counter();
    let duration_a = start_a.elapsed();
    println!("Atomic  Time: {:?}", duration_a);

    let ratio = duration_m.as_nanos() as f64 / duration_a.as_nanos() as f64;
    println!("Verdict: Atomics were {:.2}x faster than Mutex.", ratio);
    
    assert!(duration_a < duration_m, "Atomics should be faster than Mutexes!");
}