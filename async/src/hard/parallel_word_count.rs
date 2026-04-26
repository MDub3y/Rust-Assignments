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
    let lines = Arc::new(lines);
    let mut handles = vec![];

    for i in 0..4 {
        let lines_share = Arc::clone(&lines);
        let handle = thread::spawn(move || {
            let len = lines_share.len();
            
            let start = i * len / 4;
            let end = if i == 3 { len } else { (i+1) * len / 4 };
            let my_part = &lines_share[start..end];

            let mut local_map = HashMap::new();
            for line in my_part {
                for word in line.split_whitespace() {
                    *local_map.entry(word.to_string()).or_insert(0) += 1;
                }
            }
            local_map
        });
        handles.push(handle);
    }

    let mut final_map = HashMap::new();

    for handle in handles {
        let local_res = handle.join().unwrap();

        for (word, count) in local_res {
            *final_map.entry(word).or_insert(0) += count;
        }
    }

    final_map
}
