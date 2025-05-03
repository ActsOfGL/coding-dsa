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
