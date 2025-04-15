// src/helpers/contains_duplicates_within_distance.rs

pub fn contains_duplicates_within_distance(
  nums: &[i32],
  distance: i32,
) -> bool {
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
    super::*;

    #[test]
    fn test_contains_duplicates_within_distance_true() {
        let nums = vec![1, 0, 1, 1];
        let distance = 1;
        assert_eq!(
          contains_duplicates_within_distance(&nums, distance),
          true
        );
    }

    #[test]
    fn test_contains_duplicates_within_distance_false() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let distance = 2;
        assert_eq!(
          contains_duplicates_within_distance(&nums, distance),
          false
        );
    }
}