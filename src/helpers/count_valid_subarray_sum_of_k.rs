// src/helpers/count_valid_subarray_sum_of_k.rs

use std::collections::HashMap;
// This solution is O(n²) in the worst case (many entries in the hashmap).
pub fn count_valid_subarray_sum_of_k(
    logs: &[i32],
    k: i32,
    min_len: isize,
) -> i32 {
    
    let mut map: HashMap<i32, Vec<isize>> = HashMap::new();
    let mut sum: i32 = 0;
    let mut count: i32 = 0;
    
    map.entry(0).or_insert(vec![-1]); // handle subarrays starting at index 0
    
    for (i, &log) in logs.iter().enumerate() {
        let i: isize = i as isize; // from usize to isize
        sum += log;
        
        if let Some(indices) = map.get(&(sum - k)) {
            for &j in indices {
                // (i -j + 1) >= min_len as isize or
                if i - j >= min_len - 1 {
                    count += 1;
                }
            }
        }
        
        map.entry(sum).or_insert(Vec::new()).push(i);
    }
    
    count
}

// O(n) approach with a strict length
pub fn count_valid_subarray_sum_of_k_optimized(
    logs: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut prefix_sum_to_index: HashMap<i32, usize> = HashMap::new();
    let mut prefix_sum_count: HashMap<i32, i32>= HashMap::new();
    
    let mut curr_sum: i32 = 0;
    let mut count: i32 = 0;

    // Initialize: sum 0 at index -1
    prefix_sum_to_index.insert(0, usize::MAX);  // We'll treat usize::MAX as -1
    prefix_sum_count.insert(0, 1); // 1 way to form prefix sum 0 before start

    for (i, &num) in logs.iter().enumerate() {
        curr_sum += num;

        let required_sum: i32 = curr_sum - k;

        // We only count previous prefix sums that occurred far enough back
        /*
            if let Some(&earliest_index) = prefix_sum_to_index.get(&required_sum) {
                let valid = if earliest_index == usize::MAX {
                    i >= min_len
                } else {
                    i >= earliest_index + min_len
                };

                if valid {
                    // Get how many times that required sum occurred
                    let ways = match prefix_sum_count.get(&required_sum) {
                        Some(&v) => v,
                        None => 0,
                    };
                    count += ways;
                }
            }
         * The code above is the non shortcut version
         * of the code below for understanding
         */
        if let Some(&j) = prefix_sum_to_index.get(&required_sum) {
            if j == usize::MAX || i >= j + min_len {
                count += prefix_sum_count.get(&required_sum).copied().unwrap_or(0);
            }
        }

        // Update maps
        /*
            if let Some(value) = prefix_sum_count.get_mut(&curr_sum) {
                *value += 1;
            } else {
                prefix_sum_count.insert(curr_sum, 1);
            }

            // Only save first occurrence of curr_sum
            if !prefix_sum_to_index.contains_key(&curr_sum) {
                prefix_sum_to_index.insert(curr_sum, i);
            }
         * The code above is the non shortcut version
         * of the code below for understanding
         */
        prefix_sum_count
            .entry(curr_sum)
            .and_modify(|c: &mut i32| *c += 1)
            .or_insert(1);

        prefix_sum_to_index.insert(curr_sum, i);
    }

    count
}

// easier to read version of the above function
fn get_count_valid_subarray_sum_of_k(
    logs: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut prefix_sum_count: HashMap<i32, i32> = HashMap::new(); // stores how many times a prefix_sum has appeared
    let mut prefix_sum_index: HashMap<i32, usize> = HashMap::new(); // stores the *latest index* where that prefix_sum appeared

    let mut count = 0;       // how many valid subarrays we found
    let mut prefix_sum = 0;  // our running sum from left to right

    prefix_sum_count.insert(0, 1); // Base case: prefix sum = 0 has occurred once before we start
    prefix_sum_index.insert(0, usize::MAX); // Special case to simulate index = -1

    for (i, &log) in logs.iter().enumerate() {
        prefix_sum += log;

        // Let's say we want a subarray that ends at i and sums to k.
        // That means there was some prefix_sum before called (prefix_sum - k).
        let required = prefix_sum - k;

        if let Some(&j) = prefix_sum_index.get(&required) {
            // j = the index where prefix_sum - k last appeared
            // i - j = length of subarray
            if j == usize::MAX || i >= j + min_len {
                // It's valid only if subarray length is >= min_len
                if let Some(&freq) = prefix_sum_count.get(&required) {
                    count += freq;
                }
            }
        }

        // Update prefix_sum_count map
        if let Some(cnt) = prefix_sum_count.get_mut(&prefix_sum) {
            *cnt += 1;
        } else {
            prefix_sum_count.insert(prefix_sum, 1);
        }

        // Update prefix_sum_index with latest index for this sum
        prefix_sum_index.insert(prefix_sum, i);
    }

    count
}
