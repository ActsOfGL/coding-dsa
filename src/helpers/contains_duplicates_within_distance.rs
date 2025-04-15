// src/helpers/contains_duplicates_within_distance.rs

pub fn contains_duplicates_within_distance(
  nums: &[i32],
  distance: i32,
) -> bool {
    let mut map = HashMap::new();  // This line was missing
    for (i, num) in nums.iter().enumerate() {
        let index = i as i32;
        if let Some(&prev_index) = map.get(num) {
            if (index - prev_index as i32).abs() <= distance {
                return true;
            }
        }
        map.insert(num, i);
    }
}
