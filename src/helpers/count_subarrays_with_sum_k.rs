// src/helpers/count_subarrays_with_sum.rs
use std::collections::HashMap;

pub fn count_subarrays_with_sum_k_positives_only(
    logs: &[i32],
    k: i32,
) -> i32 {
    let mut left = 0;
    let mut sum = 0;
    let mut count = 0;

    for right in 0..logs.len() {
        sum += logs[right];

        while left <= right && sum > k {
            sum -= logs[left];
            left += 1;
        }

        if sum == k {
            count += 1;
        }
    }

    count
}
