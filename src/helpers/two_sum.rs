// src/helpers/two_sum.rs
pub fn two_sum(nums: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&item) = map.get(&complement) {
            return Some((item, i));
        }
        map.insert(num, i);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_two_sum_found() {
        let nums = vec![2, 7, 11, 15];
        let target= 9;
        let result = two_sum(nums, target);
        assert_eq!(result, Some((0, 1)));
    }

    #[test]
    fn test_two_sum_not_found() {
        let nums = vec![1, 2, 3];
        let target= 10;
        let result = two_sum(nums, target);
        assert_eq!(result, None);
    }

    #[test]
    fn test_two_sum_duplicate_values() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, Some((0, 1)));
    }
}