/*
  Problem 78: Thread Pool — Simple Worker Thread

  Implement a simple ThreadPool that can execute closures. The pool should
  maintain a fixed number of worker threads and use a channel to send
  jobs to workers. Implement new(size: usize) and execute<F>(&self, f: F).

  Run the tests for this problem with:
    cargo test --test thread_pool_test
*/

use std::sync::mpsc::Sender;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

pub struct ThreadPool {
    pub workers: Vec<Worker>,
    pub sender: mpsc::Sender<Job>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        let (tx, rx) = mpsc::channel::<Job>();

        let mut workers: Vec<Worker> = Vec::with_capacity(size);
        
        let receiver = Arc::new(Mutex::new(rx));

        for id in 0..size {
            let thread_receiver = Arc::clone(&receiver);
            
            let thread = thread::spawn(move || {
                loop {
                    // 1. Lock the mutex to get access to the receiver.
                    // 2. Try to receive a job.
                    let result = thread_receiver.lock().expect("Mutex poisoned").recv();

                    match result {
                        Ok(job) => {
                            // Execute the closure!
                            job();
                        }
                        Err(_) => {
                            // If recv() returns an error, the sender was dropped.
                            // This is our signal to shut down the thread.
                            break;
                        }
                    }
                }
            });

            workers.push(Worker {
                id,
                thread: Some(thread),
            });
        }

        ThreadPool {
            workers,
            sender: tx,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        
        self.sender.send(job).unwrap()
    }
}

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}
