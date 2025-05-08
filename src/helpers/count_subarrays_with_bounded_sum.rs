// src/helpers/count_subarrays_with_bounded_sum.rs
pub fn count_subarrays_with_bounded_sum(
    nums: &[i32],
    k: i32,
    min_len: usize,
) -> i32 {
    let mut prefix_sum: i32 = 0;
    let mut count: i32 = 0;
    let mut start: usize = 0;
    
    for end in 0..nums.len() {
        prefix_sum += nums[end];
        
        // Shrink the window if prefix_sum is too large
        while prefix_sum > 2 * k && start <= end {
            prefix_sum -= nums[start];
            start += 1;
        }
        
        // Count the valid subarrays ending at 'end'
        if prefix_sum >= k {
            if end - start + 1 >= min_len {
                count += (end - start - min_len + 2) as i32;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use supper::*;

    #[test]
    fn test_count_subarrays_with_bounded_sum_basic() {
        let nums: Vec<i32> = vec![2, 1, 4, 5, 1];
        let k: i32 = 5;
        let min_len: usize = 2;
        assert_eq!(count_subarrays_with_bounded_sum(&nums, k, min_len), 3);
    }
}
