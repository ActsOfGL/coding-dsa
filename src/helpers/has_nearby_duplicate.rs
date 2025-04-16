// src/helpers/has_nearby_duplicate.rs
pub fn has_nearby_duplicate(
  nums: &[i32],
  distance: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let index = i as i32;
        if let Some(&prev_index) = map.get(num) {
            if (index - prev_index as i32).abs() <= distance {
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
    fn test_has_nearby_duplicate_true() {
        let nums = vec![1, 0, 1, 1];
        let distance = 1;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
          true
        );
    }

    #[test]
    fn test_has_nearby_duplicate_false() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let distance = 2;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
          false
        );
    }

    #[test]
    fn test_empty_array() {
        let nums: Vec<i32> = vec![];
        assert_eq!(has_nearby_duplicate(&nums, 1), false);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![42];
        assert_eq!(has_nearby_duplicate(&nums, 1), false);
    }

    #[test]
    fn test_exact_distance_match() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(has_nearby_duplicate(&nums, 3), true);
    }

    #[test]
    fn test_distance_zero() {
        let nums = vec![1, 2, 3, 1];
        assert_eq!(has_nearby_duplicate(&nums, 0), false);
    }
}