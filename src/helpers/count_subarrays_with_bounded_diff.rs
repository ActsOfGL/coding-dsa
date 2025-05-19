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

/**
 * @returns: the length of the longest subarray with a maximum difference of `k` at most
 */
fn get_subarray_length_bound_diff_at_most_k(nums: &[i32], k: i32) -> usize {
    let mut max_deque= VecDeque::new();
    let mut min_deque= VecDeque::new();
    let mut left = 0;
    let mut max_len = 0;

    for (right, &num) in nums.iter().enumerate() {
        // Maintain decreasing order for max_deque
        while let Some(&last) = max_deque.back() {
            if nums[last] <= num {
                max_deque.pop_back();
            } else {
                break;
            }
        }
        max_deque.push_back(right);

        // Maintain increasing order for min_deque
        while let Some(&last) = min_deque.back() {
            if nums[last] >= num {
                min_deque.pop_back();
            } else {
                break;
            }
        }
        min_deque.push_back(right);

        // Shrink window if the current max - min exceeds k
        while nums[*max_deque.front().unwrap()] - nums[*min_deque.front().unwrap()] > k {
            left += 1;
            // Remove out-of-bound indices from the deques
            if *max_deque.front().unwrap() < left {
                max_deque.pop_front();
            }
            if *min_deque.front().unwrap() < left {
                min_deque.pop_front();
            }
        }

        // Update max_len with the current valid window size
        max_len = max_len.max(right - left + 1);
    }

    max_len
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
            3,
        );
    }

    #[test]
    fn test_count_subarrays_with_bounded_diff_zero_output() {
        let nums: Vec<i32> = vec![1, 2];
        let k: i32 = 5;
        let min_len: usize = 3;
        assert_eq!(
            count_subarrays_with_bounded_diff(&nums, k, min_len),
            0,
        );
    }
}
