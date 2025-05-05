// src/helpers/count_valid_subarray_max_minus_min.rs
use std::collections::VecDeque;

pub fn count_valid_subarray_max_minus_min(
    nums: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut count = 0;
    let mut max_deque = VecDeque::new();
    let mut min_deque = VecDeque::new();
    let mut start = 0; // starting index

    for end in 0..nums.len() {
        while let Some(&last) = max_deque.back() {
            if nums[end] > nums[last] {
                max_deque.pop_back();
            } else {
                break;
            }
        }

        max_deque.push_back(end);

        while let Some(&last) = min_deque.back() {
            if nums[end] < nums[last] {
                min_deque.pop_back();
            } else {
                break;
            }
        }

        min_deque.push_back(end);

        while nums[*max_deque.front().unwrap()] - nums[*min_deque.fron().unwrap()] > k {
            if max_deque.front() == Some(&start) {
                max_deque.pop_front();
            }
            if min_deque.front() == Some(&start) {
                min_deque.pop_front();
            }
            start += 1;
        }

        if end + 1 - start >= min_len {
            count += (end + 1 - start + 1 - min_len + 1) as i32;
        }
    }
    count
}