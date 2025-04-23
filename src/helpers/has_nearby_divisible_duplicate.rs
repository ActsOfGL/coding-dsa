// src/helpers/has_nearby_divisible_duplicate
pub fn has_nearby_divisible_duplicate(
    nums: &[i32],
    distance: i32,
    divisor: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<i32, usize> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if num % divisor != 0 {
            continue;
        }
        if let Some(&prev_index) = map.get(&num) {
            if i - prev_index <= distance as usize {
                return true;
            }
        }
        map.insert(num, i);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_true_when_duplicate_divisible_within_distance() {
        let nums: Vec<i32> = vec![3, 0, 3, 3, 2, 1, 5, 6, 4];
        let distance: i32 = 1;
        let divisor: i32 = 3;

        let result: bool = has_nearby_divisible_duplicate(&nums, distance, divisor);

        assert!(result);
    }

    #[test]
    fn returns_false_when_no_duplicate_divisible_within_distance() {
        let nums: Vec<i32> = vec![1, 3, 6, 9];
        let distance: i32 = 1;
        let divisor: i32 = 3;

        let result: bool = has_nearby_divisible_duplicate(&nums, distance, divisor);

        assert!(!result);
    }
}