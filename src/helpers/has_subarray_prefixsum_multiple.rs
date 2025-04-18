// src/helpers/has_subarray_prefixsum_multiple.rs
pub fn has_subarray_prefixsum_multiple(
  nums: &[i32],
  target: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut prefix_sum = 0;
    map.insert(0, 0 as usize);
    for (i, &num) in nums.iter().enumerate() {
        if (i == 0 && temp == 0) {
           continue;
        }
        prefix_sum += num;
        let remainder = ((prefix_sum % target) + target) % target;
        if let Some(&prev_index) = map.get(&remainder) {
            if (i as i32 - (prev_index as i32 + 1)) >= 2 {
                return true;
            }
        }
        map.insert(remainder, i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subarray_prefixsum_multiple_true() {
        let nums = vec![5, 3, 1, 7];
        let target = 6;
        assert!(has_subarray_prefixsum_multiple(&nums, target));
    }

    #[test]
    fn test_has_subarray_prefixsum_multiple_false() {
        let nums = vec![2, 4, 3];
        let target = 5;
        assert!(!has_subarray_prefixsum_multiple(&nums, target));  
    }
}