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

    let mut max_sum = current_sum;

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
