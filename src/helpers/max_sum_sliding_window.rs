// src/helpers/max_sum_sliding_window.rs
pub fn max_sum_sliding_window(
    nums: &[i32],
    k: usize,
) -> i32 {
    if nums.len() < k || k == 0 {
        return 0;
    }

    let mut current_sum: i32 = nums.iter().take(k).sum();
    /*
      * code equivalent for nums.iter().take(k).sum();
      * the one above uses Rust's built-in max function and idiomatic syntax
      * let mut current_sum = 0;
      * for i in 0..k {
      *     current_sum += nums[i];
      * }
      */

    let mut max_sum: i32 = current_sum;

    for i in k..nums.len() {
        current_sum = current_sum + nums[i] - nums[i - k];

        max_sum = max_sum.max(current_sum);
        /*
         * code equivalent for max_sum.max(current_sum)
         * the one above uses Rust's built-in max function and idiomatic syntax
         * if current_sum > max_sum {
         *     max_sum = current_sum;
         * }
        */
    }

    max_sum
}

// addaptive version
pub fn max_sum_sliding_window_of_k_adaptive(
    nums: &[i32],
    k: usize,
) -> i32 {
    let mut left: usize = 0;
    let mut sum: i32 = 0;
    let mut max_sum: i32 = 0;

    for right in 0..nums.len() {
        sum += nums[right];

        if right - left + 1 > k { // depends if > or < or with =
            sum -= nums[left];
            left += 1;
        }
        max_sum = max_sum.max(sum);
    }
    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_sliding_window() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 3;
        let output: i32 = max_sum_sliding_window(&nums, k);
        let expected = 9;
        assert_eq!(
            max_sum_sliding_window(
                &nums,
                k,
            ),
            9,
        );
    }

    #[test]
    fn test_max_sum_sliding_window_zero_target() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 0;
        assert_eq!(
            max_sum_sliding_window(
                &nums,
                k,
            ),
            0,
        );
    }

    #[test]
    fn test_max_sum_sliding_window_smaller_array() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 15;
        assert_eq!(
            max_sum_sliding_window(
                &nums,
                k,
            ),
            0,
        );
    }

    #[test]
    fn test_max_sum_sliding_window_of_k_adaptive() {
        let logs: Vec<i32> = vec![5, 1, 3, 7, 2, 6, 4];
        let k: usize = 15;
        assert_eq!(
            max_sum_sliding_window_of_k_adaptive(
                &logs,
                k,
            ),
            15,
        ); // subarray [3,7,2] -> sum = 12, [7,2,6] -> sum = 15
    }

    #[test]
    fn test_max_sum_sliding_window_of_k_adaptive_zero() {
        let logs: Vec<i32> = vec![5, 1, 3, 7, 2, 6, 4];
        let k: usize = 0;
        assert_eq!(
            max_sum_sliding_window_of_k_adaptive(
                &logs,
                k,
            ),
            0,
        ); // subarray [3,7,2] -> sum = 12, [7,2,6] -> sum = 15
    }

    #[test]
    fn test_max_sum_sliding_window_empty_array() {
        let nums: Vec<i32> = Vec::new();
        let k: usize = 15;
        assert_eq!(
            max_sum_sliding_window(
                &nums,
                k,
            ),
            0,
        );
    }

    #[test]
    fn test_max_sum_sliding_window_of_k_adaptive_empty_array() {
        let nums: Vec<i32> = Vec::new(); // vec![];
        let k: usize = 5;
        assert_eq!(
            max_sum_sliding_window_of_k_adaptive(
                &nums,
                k,
            ),
            0,
        );
    }
}