// src/helpers/count_valid_subarray_sum_of_k.rs

use std::collections::HashMap;
// This solution is O(n²) in the worst case (many entries in the hashmap).
pub fn count_valid_subarray_sum_of_k(
    logs: &[i32],
    k: i32,
    min_len: i32,
) -> i32 {
    
    let mut map = HashMap::new();
    let mut sum = 0;
    let mut count = 0;
    
    map.entry(0).or_insert(vec![-1usize]); // handle subarrays starting at index 0
    
    for (i, &log) in logs.iter().enumerate() {
        sum += log;
        
        if let Some(indices) = map.get(&(sum - k)) {
            for &j in indices {
                if i - j >= (min_len - 1) as usize {
                    count += 1;
                }
            }
        }
        
        map.entry(sum).or_insert(Vec::new()).push(i);
    }
    
    count
}
