/*
  Problem 75: Shared State — RwLock Configuration

  Define a Config struct with a HashMap<String, String>. Wrap it in Arc<RwLock<Config>>.
  Write a function that spawns 5 reader threads and 1 writer thread.
  Readers should read a specific key, while the writer updates it.
  Return the final value of the configuration key.

  Run the tests for this problem with:
    cargo test --test rwlock_config_test
*/

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::thread;

pub struct Config {
    pub settings: HashMap<String, String>,
}

pub fn update_and_read_config() -> String {
    let mut initial_settings = HashMap::new();
    initial_settings.insert("status".to_string(), "starting".to_string());

    let config = Arc::new(RwLock::new(Config { settings: initial_settings }));
    let mut handles = vec![];

    for _ in 0..5 {
        let config_clone = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let read_guard = config_clone.read().unwrap();
            let _val = read_guard.settings.get("status");
        });
        handles.push(handle);
    }

    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        let mut write_guard = config_clone.write().unwrap();
        write_guard.settings.insert("status".to_string(), "updated".to_string());
    });
    handles.push(handle);

    for handle in handles {
        handle.join().unwrap();
    }

    let final_guard = config.read().unwrap();
    final_guard.settings.get("status").unwrap().clone()
}
