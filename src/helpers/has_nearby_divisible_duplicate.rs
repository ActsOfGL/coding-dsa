// src/helpers/has_nearby_divisible_duplicate
pub fn has_nearby_divisible_duplicate(
    nums: &[32],
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
