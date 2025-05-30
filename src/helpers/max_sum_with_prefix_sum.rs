// src/helpers/max_sum_with_prefix_sum.rs
pub fn max_sum_with_prefix_sum(nums: &[i32], k: usize) -> i32 {
    if nums.len() < k || k == 0 {
        return 0;
    }

    let mut prefix_sums: Vec<i32> = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix_sums[i + 1] = prefix_sums[i] + nums[i]; // Build prefix sum
    }

    let mut max_sum: i32 = 0;
    for i in k..=nums.len() {
        let window_sum: i32 = prefix_sums[i] - prefix_sums[i - k]; // Get window sum
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }

    max_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_sum_with_prefix_sum() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 3;
        assert_eq!(
            max_sum_with_prefix_sum(
                &nums,
                k,
            ),
            9,
        );
    }

    #[test]
    fn test_max_sum_with_prefix_sum_zero_target() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 0;
        assert_eq!(
            max_sum_with_prefix_sum(
                &nums,
                k,
            ),
            0,
        );
    }

    #[test]
    fn test_max_sum_with_prefix_sum_smaller_array() {
        let nums: Vec<i32> = vec![1, 2, 1, 5, 1, 3, 1];
        let k: usize = 15;
        assert_eq!(
            max_sum_with_prefix_sum(
                &nums,
                k,
            ),
            0,
        );
    }

    #[test]
    fn test_max_sum_with_prefix_sum_empty_array() {
        let nums: Vec<i32> = vec![];
        let k: usize = 15;
        assert_eq!(max_sum_with_prefix_sum(&nums, k), 0);
    }
}