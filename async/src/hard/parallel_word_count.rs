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
use std::thread;

pub fn parallel_word_count(lines: Vec<String>) -> HashMap<String, usize> {
    let mut finalMap: HashMap<String, usize> = HashMap::new();

    let len = lines.len();

    let part1 = lines[0..len / 4].to_vec();
    let part2 = lines[len / 4..len / 2].to_vec();
    let part3 = lines[len / 2..3 * len / 4].to_vec();
    let part4 = lines[3 * len / 4..].to_vec();

    let mut handles = vec![];

    let handle1 = thread::spawn(move || {
        let mut local_map: HashMap<String, usize> = HashMap::new();

        for line in part1 {
            for word in line.split_whitespace() {
                if let Some(val) = local_map.get_mut(word) {
                    *val += 1;
                } else {
                    local_map.insert(word.to_string(), 1);
                }
            }
        }
        local_map
    });
    handles.push(handle1);

    let handle2 = thread::spawn(move || {
        let mut local_map: HashMap<String, usize> = HashMap::new();
        for line in part2 {
            for word in line.split_whitespace() {
                if let Some(val) = local_map.get_mut(word) {
                    *val += 1;
                } else {
                    local_map.insert(word.to_string(), 1);
                }
            }
        }
        local_map
    });
    handles.push(handle2);

    // Thread 3
    let handle3 = thread::spawn(move || {
        let mut local_map: HashMap<String, usize> = HashMap::new();
        for line in part3 {
            for word in line.split_whitespace() {
                if let Some(val) = local_map.get_mut(word) {
                    *val += 1;
                } else {
                    local_map.insert(word.to_string(), 1);
                }
            }
        }
        local_map
    });
    handles.push(handle3);

    // Thread 4
    let handle4 = thread::spawn(move || {
        let mut local_map: HashMap<String, usize> = HashMap::new();
        for line in part4 {
            for word in line.split_whitespace() {
                if let Some(val) = local_map.get_mut(word) {
                    *val += 1;
                } else {
                    local_map.insert(word.to_string(), 1);
                }
            }
        }
        local_map
    });
    handles.push(handle4);

    for handle in handles {
        // Wait for the thread to finish and get its local_map
        let local_res = handle.join().unwrap();

        // Merge the local counts into the finalMap
        for (word, count) in local_res {
            if let Some(val) = finalMap.get_mut(&word) {
                *val += count;
            } else {
                finalMap.insert(word, count);
            }
        }
    }

    finalMap
}
