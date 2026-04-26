/*
  Problem 81: Parallel Word Count

  Write a function that takes a Vec<String> (lines of text) and counts the
  occurrences of words in parallel. Split the lines among 4 threads. Each thread
  computes a local HashMap, and then the main thread merges them into a final
  HashMap<String, usize>.

  Run the tests for this problem with:
    cargo test --test parallel_word_count_test
*/

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

pub fn parallel_word_count(lines: Vec<String>) -> HashMap<String, usize> {
    let len = lines.len();
    if len == 0 {
        return HashMap::new();
    }

    let num_threads = 4;
    let chunk_size = (len + num_threads - 1) / num_threads;

    let partial_results = thread::scope(|s| {
        let mut handles = vec![];

        for chunk in lines.chunks(chunk_size) {
            let handle = s.spawn(move || {
                let mut local_map = HashMap::new();
                for line in chunk {
                    for word in line.split_whitespace() {
                        // The map needs to own the String to store it
                        *local_map.entry(word.to_string()).or_insert(0) += 1;
                    }
                }
                local_map
            });
            handles.push(handle);
        }

        handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect::<Vec<HashMap<String, usize>>>()
    });

    let mut final_map = HashMap::new();
    for local_map in partial_results {
        for (word, count) in local_map {
            *final_map.entry(word).or_insert(0) += count;
        }
    }

    final_map
}
