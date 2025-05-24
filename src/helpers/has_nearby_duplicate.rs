// src/helpers/has_nearby_duplicate.rs
pub fn has_nearby_duplicate(
  nums: &[i32],
  distance: i32,
) -> bool {
    use std::collections::HashMap;
    let mut map: HashMap<&i32, usize> = HashMap::new();
    for (i, num) in nums.iter().enumerate() {
        let index: i32 = i as i32;
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
        let nums: Vec<i32> = vec![1, 0, 1, 1];
        let distance: i32 = 1;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
            true,
        );
    }

    #[test]
    fn test_has_nearby_duplicate_false() {
        let nums: Vec<i32> = vec![1, 2, 3, 1, 2, 3];
        let distance: i32 = 2;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
            false,
        );
    }

    #[test]
    fn test_has_nearby_duplicate_empty_array() {
        let nums: Vec<i32> = vec![];
        let distance: i32 = 1;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
            false,
        );
    }

    #[test]
    fn test_has_nearby_duplicate_single_element() {
        let nums: Vec<i32> = vec![42];
        let distance: i32 = 1;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
            false,
        );
    }

    #[test]
    fn test_has_nearby_duplicate_exact_distance_match() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        let distance = 3;
        assert_eq!(
            has_nearby_duplicate(&nums, distance),
            true,
        );
    }

    #[test]
    fn test_distance_zero() {
        let nums: Vec<i32> = vec![1, 2, 3, 1];
        assert_eq!(
            has_nearby_duplicate(&nums, 0),
            false,
        );
    }
}