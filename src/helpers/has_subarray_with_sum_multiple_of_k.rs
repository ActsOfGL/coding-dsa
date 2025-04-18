// src/helpers/has_subarray_prefixsum_multiple.rs
pub fn has_subarray_with_sum_multiple_of_k(
  nums: &[i32],
  target: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, isize> = HashMap::new();
    let mut prefix_sum = 0;
    map.insert(prefix_sum, -1);
    for (i, &num) in nums.iter().enumerate() {
        prefix_sum += num;
        let remainder = ((prefix_sum % target) + target) % target;
        if let Some(&prev_index) = map.get(&remainder) {
            if i as isize - prev_index >= 2 {
                return true;
            }
        } else {
            map.entry(remainder).or_insert(i as isize);
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subarray_with_sum_multiple_of_k_true() {
        let nums = vec![2, 4, 3];
        let target = 6;
        assert!(has_subarray_with_sum_multiple_of_k(&nums, target));
    }

    #[test]
    fn test_has_subarray_with_sum_multiple_of_k_false() {
        let nums = vec![2, 4, 3];
        let target = 5;
        assert!(!has_subarray_with_sum_multiple_of_k(&nums, target));  
    }
}