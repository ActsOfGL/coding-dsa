// src/helpers/two_sum.rs
pub fn two_sum(
    nums: &[i32],
    target: i32,
) -> Option<(usize, usize)> {
    use std::collections::HashMap;
    let mut map: HashMap<&i32, usize>  = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let complement: i32 = target - num;
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
        let nums: Vec<i32> = vec![2, 7, 11, 15];
        let target: i32 = 9;
        let result: Option<(usize, usize)> = two_sum(&nums, target);
        let expected: Option<(usize, usize)> = Some((0, 1));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_sum_not_found() {
        let nums: Vec<i32> = vec![1, 2, 3];
        let target: i32 = 10;
        let result: Option<(usize, usize)> = two_sum(&nums, target);
        let expected: Option<(usize, usize)> = None;
        assert_eq!(result, expected);
    }

    #[test]
    fn test_two_sum_empty_array() {
        let nums: Vec<i32> = Vec::new();
        let target: i32 = 10;
        let result: Option<(usize, usize)> = two_sum(&nums, target);
        let expected = None;
        assert_eq!(result, None);
    }

    #[test]
    fn test_two_sum_duplicate_values() {
        let nums: Vec<i32> = vec![3, 3];
        let target: i32 = 6;
        let result: Option<(usize, usize)> = two_sum(&nums, target);
        let expected: Option<(usize, usize)> = Some((0, 1));
        assert_eq!(result, expected);
    }
}