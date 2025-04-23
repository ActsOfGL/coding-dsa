// src/helpers/has_subarray_prefixsum_multiple.rs
pub fn has_subarray_with_sum_multiple_of_k(
  nums: &[i32],
  target: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, isize> = HashMap::new();
    let mut prefix_sum: i32 = 0;
    map.insert(prefix_sum, -1);
    for (i, &num) in nums.iter().enumerate() {
        prefix_sum += num;
        let remainder: i32 = ((prefix_sum % target) + target) % target;
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

/*
 * same function but idiomatic rust
 * uses usize instead of isize
 * no need to insert before the loop
 * no else statement
 * not a perfect solution, it missed an edge case for map.inser(0, 0) scenario
 * return true
 */
pub fn has_subarray_with_sum_multiple_of_k_other_version(
    nums: &[i32],
    k: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();
    let mut prefix_sum: i32 = 0;

    for (i, &num) in nums.iter().enumerate() {
        prefix_sum += num;
        let remainder: i32 = ((prefix_sum % k) + k) % k;
        let entry: &mut usize = map.entry(remainder).or_insert(i);
        if i - *entry >= 2 {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_subarray_with_sum_multiple_of_k_true() {
        let nums: Vec<i32> = vec![2, 4, 3];
        let target: i32 = 6;
        assert!(has_subarray_with_sum_multiple_of_k(&nums, target));
    }

    #[test]
    fn test_has_subarray_with_sum_multiple_of_k_false() {
        let nums: Vec<i32> = vec![2, 4, 3];
        let target: i32 = 5;
        assert!(!has_subarray_with_sum_multiple_of_k(&nums, target));  
    }
}