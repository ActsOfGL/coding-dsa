// src/helpers/max_sum_sliding_window.rs
pub fn max_sum_sliding_window(
    nums: &[i32],
    k: usize,
) -> i32 {
    if nums.len() < k || k == 0 {
        return 0;
    }

    let mut current_sum = 0;
    for i in 0..k {
        current_sum += nums[i]; // Sum first window
    }

    let mut max_sum = current_sum;

    for i in k..nums.len() {
        current_sum = current_sum + nums[i] - nums[i - k]; // Slide window
        if current_sum > max_sum {
            max_sum = current_sum;
        }
    }

    max_sum
}
