/*
  Problem 100: Async Task Scheduler

  Implement a TaskScheduler that can schedule async closures to run after
  a specific delay. It should use a PriorityQueue (or sorted Vec) to keep track
  of tasks and a background tokio task to execute them when their time comes.

  Run the tests for this problem with:
    cargo test --test task_scheduler_test
*/

use tokio::time::{sleep, Duration, Instant};
use std::sync::{Arc, Mutex};

pub struct TaskScheduler {
    pub tasks: Arc<Mutex<Vec<(Instant, Box<dyn FnOnce() + Send + 'static>)>>>,
}

impl TaskScheduler {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(Vec::new()))
        }
    }

    pub fn schedule<F>(&self, delay: Duration, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let run_at = Instant::now() + delay;
        let mut tasks = self.tasks.lock().unwrap();
        
        tasks.push((run_at, Box::new(f)));
        
        tasks.sort_by(|a, b| b.0.cmp(&a.0));
    }

    pub fn start(&self) {
        let tasks_clone = Arc::clone(&self.tasks);

        tokio::spawn(async move {
            loop {
                let task_to_run = {
                    let mut tasks = tasks_clone.lock().unwrap();
                    
                    if let Some((time, _)) = tasks.last() {
                        if Instant::now() >= *time {
                            let (_, f) = tasks.pop().unwrap();
                            Some(f)
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(f) = task_to_run {
                    f();
                } else {
                    sleep(Duration::from_millis(10)).await;
                }
            }
        });
    }
}
