// src/helpers/count_valid_subarray_sum_of_k.rs

use std::collections::HashMap;
// This solution is O(n²) in the worst case (many entries in the hashmap).
pub fn count_valid_subarray_sum_of_k(
    logs: &[i32],
    k: i32,
    min_len: i32,
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
                if i - j >= min_len as isize - 1 {
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
