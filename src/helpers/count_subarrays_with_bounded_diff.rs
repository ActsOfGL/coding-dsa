// src/helpers/count_subarrays_with_bounded_diff.rs
use std::collections::VecDeque;

pub fn count_subarrays_with_bounded_diff(
    nums: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut count: i32 = 0;
    let mut max_deque: VecDeque<usize> = VecDeque::new();
    let mut min_deque: VecDeque<usize> = VecDeque::new();
    let mut start: usize = 0; // starting index

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

        while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > k {
            if max_deque.front() == Some(&start) {
                max_deque.pop_front();
            }
            if min_deque.front() == Some(&start) {
                min_deque.pop_front();
            }
            start += 1;
        }

        if end + 1 - start >= min_len {
            count += ((end - start + 1) - min_len + 1) as i32;
        }
    }
    count

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_subarrays_with_bounded_diff() {
        let nums: Vec<i32> = vec![4, 2, 5, 1, 6];
        let k: i32 = 3;
        let min_len: usize = 2;
        assert_eq!(
            count_subarrays_with_bounded_diff(&nums, k, min_len),
            3
        );
    }

    #[test]
    fn test_count_subarrays_with_bounded_diff_zero_output() {
        let nums = vec![1, 2];
        let k = 5;
        let min_len = 3;
        assert_eq!(
            count_subarrays_with_bounded_diff(&nums, k, min_len),
            0
        );
    }
}
