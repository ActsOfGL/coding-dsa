// src/helpers/max_sum_with_prefix_sum.rs
pub fn max_sum_with_prefix_sum(nums: &[i32], k: usize) -> i32 {
    if nums.len() < k || k == 0 {
        return 0;
    }

    let mut prefix_sums = vec![0; nums.len() + 1];
    for i in 0..nums.len() {
        prefix_sums[i + 1] = prefix_sums[i] + nums[i]; // Build prefix sum
    }

    let mut max_sum = 0;
    for i in k..=nums.len() {
        let window_sum = prefix_sums[i] - prefix_sums[i - k]; // Get window sum
        if window_sum > max_sum {
            max_sum = window_sum;
        }
    }

    max_sum
}